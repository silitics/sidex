/**
 * Runtime for Sidex's validation extension.
 *
 * The TypeScript codegen emits calls into this module for each
 * `#[validate(...)]` rule on a generated type. A validator returns a
 * {@link ValidationReport} of zero or more {@link ValidationError}s rather
 * than throwing — multiple errors per call surface every problem at once,
 * which is what UI forms want.
 *
 * The shape of {@link ValidationError}, the codes, the paths, and the
 * default messages mirror `sidex::validation` in Rust. A code/path tuple from
 * a TypeScript driver and a Rust driver are byte-for-byte comparable.
 */

// ---- Path -----------------------------------------------------------------

/** A path segment. */
export type PathSegment =
  | { readonly kind: "field"; readonly name: string }
  | { readonly kind: "index"; readonly index: number }
  | { readonly kind: "key"; readonly key: string }

/**
 * Path identifying the location of a value within the validated structure,
 * like `devices[3].timeoutMs`. Reported on every {@link ValidationError} so
 * frontends can map errors back to specific fields, sequence indices, or map
 * keys. Fields appear under their JSON names. The root is the empty path.
 */
export class Path {
  private readonly segments: readonly PathSegment[]

  private constructor(segments: readonly PathSegment[]) {
    this.segments = segments
  }

  static root(): Path {
    return new Path([])
  }

  field(name: string): Path {
    return new Path([...this.segments, { kind: "field", name }])
  }

  index(idx: number): Path {
    return new Path([...this.segments, { kind: "index", index: idx }])
  }

  key(key: string): Path {
    return new Path([...this.segments, { kind: "key", key }])
  }

  isRoot(): boolean {
    return this.segments.length === 0
  }

  toString(): string {
    let out = ""
    for (const seg of this.segments) {
      if (seg.kind === "index") {
        out += `[${seg.index}]`
      } else {
        const name = seg.kind === "field" ? seg.name : seg.key
        out += out === "" ? name : `.${name}`
      }
    }
    return out
  }
}

// ---- Errors / report ------------------------------------------------------

/**
 * One validation failure. `code` is a stable identifier the application
 * can map to a localized string; `message` is the human-readable form
 * (either the default keyed on `code` or a user override from the
 * `message = "..."` attribute).
 */
export interface ValidationError {
  readonly path: string
  readonly code: string
  readonly message: string
}

/**
 * Bag of validation errors with composition primitives. An empty report
 * means "valid".
 */
export class ValidationReport {
  private readonly _errors: ValidationError[]

  constructor(errors: ValidationError[] = []) {
    this._errors = errors
  }

  static ok(): ValidationReport {
    return new ValidationReport()
  }

  static fromError(error: ValidationError): ValidationReport {
    return new ValidationReport([error])
  }

  get errors(): readonly ValidationError[] {
    return this._errors
  }

  isOk(): boolean {
    return this._errors.length === 0
  }

  isErr(): boolean {
    return this._errors.length > 0
  }

  push(error: ValidationError): void {
    this._errors.push(error)
  }

  /** Drain `other`'s errors into this report. */
  merge(other: ValidationReport): void {
    for (const e of other._errors) this._errors.push(e)
  }

  toString(): string {
    return this._errors
      .map((e) => `${e.code} at ${e.path}: ${e.message}`)
      .join("\n")
  }
}

/** Implemented by every generated type that has at least one rule. */
export interface Validate {
  validateAt(path: Path): ValidationReport
}

export function validate(value: Validate): ValidationReport {
  return value.validateAt(Path.root())
}

// ---- Default messages -----------------------------------------------------

const DEFAULT_MESSAGES: Record<string, string> = {
  min: "Value is below the minimum.",
  max: "Value is above the maximum.",
  eq: "Value does not match the expected value.",
  ne: "Value matches a forbidden value.",
  min_size: "Value is below the minimum size.",
  max_size: "Value exceeds the maximum size.",
  eq_size: "Value does not have the expected size.",
  ne_size: "Value has a forbidden size.",
  regex: "Value does not match the required pattern.",
}

export function defaultMessage(code: string): string {
  return DEFAULT_MESSAGES[code] ?? "Value is not valid."
}

function pickMessage(message: string | null, code: string): string {
  if (message !== null) return message
  return defaultMessage(code)
}

function reportOne(
  path: Path,
  code: string,
  message: string | null,
): ValidationReport {
  return ValidationReport.fromError({
    path: path.toString(),
    code,
    message: pickMessage(message, code),
  })
}

// ---- Numbers --------------------------------------------------------------

/**
 * Converts the JSON representation of a number into a comparable value.
 *
 * 64-bit integers beyond `Number.MAX_SAFE_INTEGER` are decimal strings and
 * become `bigint`s, so comparisons stay exact. Non-finite floats are the strings `"NaN"`, `"Infinity"`, and
 * `"-Infinity"`. Other values are returned unchanged.
 */
export function numeric(value: unknown): unknown {
  if (typeof value !== "string") return value
  if (/^-?[0-9]+$/.test(value)) return BigInt(value)
  switch (value) {
    case "NaN":
      return NaN
    case "Infinity":
    case "+Infinity":
      return Infinity
    case "-Infinity":
      return -Infinity
    default:
      return value
  }
}

/** Equality that treats `bigint`s and numbers with the same value as equal. */
function equal(a: unknown, b: unknown): boolean {
  if (typeof a === "bigint" || typeof b === "bigint") {
    // Loose equality compares `bigint`s and numbers by value.
    return a == b
  }
  return a === b
}

// ---- Comparison helpers ---------------------------------------------------
//
// Each helper has the same signature shape as the Rust runtime:
//
//     fn min_inclusive<T>(value: T, bound: T, code, message, path);
//
// The bound is provided by the codegen as a literal of the same type as
// the value (numbers stay numbers; counts come back from `charCount` /
// `byteCount` / `itemCount` / `entryCount` as `number`).

export function minInclusive<T>(
  value: T,
  bound: T,
  code: string,
  message: string | null,
  path: Path,
): ValidationReport {
  return value >= bound ? ValidationReport.ok() : reportOne(path, code, message)
}

export function minExclusive<T>(
  value: T,
  bound: T,
  code: string,
  message: string | null,
  path: Path,
): ValidationReport {
  return value > bound ? ValidationReport.ok() : reportOne(path, code, message)
}

export function maxInclusive<T>(
  value: T,
  bound: T,
  code: string,
  message: string | null,
  path: Path,
): ValidationReport {
  return value <= bound ? ValidationReport.ok() : reportOne(path, code, message)
}

export function maxExclusive<T>(
  value: T,
  bound: T,
  code: string,
  message: string | null,
  path: Path,
): ValidationReport {
  return value < bound ? ValidationReport.ok() : reportOne(path, code, message)
}

export function eq<T>(
  value: T,
  expected: T,
  code: string,
  message: string | null,
  path: Path,
): ValidationReport {
  return equal(value, expected)
    ? ValidationReport.ok()
    : reportOne(path, code, message)
}

export function ne<T>(
  value: T,
  forbidden: T,
  code: string,
  message: string | null,
  path: Path,
): ValidationReport {
  return !equal(value, forbidden)
    ? ValidationReport.ok()
    : reportOne(path, code, message)
}

/** Comparison operator of a rule. */
export type CmpOp = "<" | "<=" | "==" | "!=" | ">=" | ">"

/**
 * Checks a chained comparison, which holds if each comparison of adjacent
 * operands holds.
 */
export function holds(
  operands: readonly unknown[],
  ops: readonly CmpOp[],
): boolean {
  return ops.every((op, i) => {
    const a = operands[i] as any
    const b = operands[i + 1] as any
    switch (op) {
      case "<":
        return a < b
      case "<=":
        return a <= b
      case "==":
        return equal(a, b)
      case "!=":
        return !equal(a, b)
      case ">=":
        return a >= b
      case ">":
        return a > b
    }
  })
}

/** Reports a violation with the given code unless the rule holds. */
export function check(
  holds: boolean,
  code: string,
  message: string | null,
  path: Path,
): ValidationReport {
  return holds ? ValidationReport.ok() : reportOne(path, code, message)
}

// ---- Accessors ------------------------------------------------------------

/**
 * Length of a string in Unicode code points. Pinned across targets — the
 * Rust runtime uses `chars().count()`, this uses spread iteration which
 * gives the same answer (one code point per surrogate pair, not two
 * UTF-16 units like JavaScript's native `String.length`).
 */
export function charCount(s: string): number {
  let n = 0
  // Spread iteration walks code points (`for…of` does the same).
  for (const _ of s) n++
  return n
}

/** Length of a string in UTF-8 bytes. */
export function byteCount(s: string): number {
  // `TextEncoder` is available in browsers and Node since v11. Avoids
  // pulling in `Buffer` (Node-only).
  return new TextEncoder().encode(s).length
}

/** Number of elements in a sequence. */
export function itemCount(seq: readonly unknown[]): number {
  return seq.length
}

/** Number of entries in a map (plain object keyed by strings). */
export function entryCount(map: Record<string, unknown>): number {
  return Object.keys(map).length
}

// ---- Regex ----------------------------------------------------------------

const REGEX_CACHE = new Map<string, RegExp>()

/** Cache lookup keyed on pattern source. Codegen emits the source string. */
function compiledRegex(pattern: string): RegExp {
  let r = REGEX_CACHE.get(pattern)
  if (r === undefined) {
    r = new RegExp(pattern)
    REGEX_CACHE.set(pattern, r)
  }
  return r
}

export function regexMatch(
  s: string,
  pattern: string,
  code: string,
  message: string | null,
  path: Path,
): ValidationReport {
  return compiledRegex(pattern).test(s)
    ? ValidationReport.ok()
    : reportOne(path, code, message)
}
