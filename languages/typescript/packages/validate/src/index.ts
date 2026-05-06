/**
 * Runtime for Sidex's validation extension.
 *
 * The TypeScript codegen emits calls into this module for each
 * `#[validate(...)]` rule on a generated type. A validator returns a
 * {@link ValidationReport} of zero or more {@link ValidationError}s rather
 * than throwing — multiple errors per call surface every problem at once,
 * which is what UI forms want.
 *
 * The shape of {@link ValidationError}, the helper signatures, and the
 * default-message vocabulary mirror the Rust runtime in
 * `crates/sidex-validate` exactly. A code/path tuple from a TypeScript
 * driver and a Rust driver are byte-for-byte comparable.
 */

// ---- Path -----------------------------------------------------------------

/** A JSON-pointer-style path segment. */
export type PathSegment =
  | { readonly kind: "field"; readonly name: string }
  | { readonly kind: "index"; readonly index: number }
  | { readonly kind: "key"; readonly key: string }

/**
 * JSON-pointer-style path identifying the location of a value within the
 * validated structure. Reported on every {@link ValidationError} so
 * frontends can map errors back to specific fields, sequence indices, or
 * map keys.
 */
export class Path {
  private constructor(private readonly segments: readonly PathSegment[]) {}

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
    if (this.segments.length === 0) return "/"
    let out = ""
    for (const seg of this.segments) {
      out += "/"
      if (seg.kind === "index") {
        out += seg.index.toString()
      } else {
        const raw = seg.kind === "field" ? seg.name : seg.key
        // RFC 6901: `~` → `~0`, `/` → `~1`. Escape `~` first to avoid
        // double-encoding the resulting `~1`.
        out += raw.replace(/~/g, "~0").replace(/\//g, "~1")
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
  min_size: "Value is below the minimum size.",
  max_size: "Value exceeds the maximum size.",
  eq: "Value does not match the expected value.",
  ne: "Value matches a forbidden value.",
  regex: "Value does not match the required pattern.",
  predicate: "Value does not satisfy the predicate.",
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
  return value === expected ? ValidationReport.ok() : reportOne(path, code, message)
}

export function ne<T>(
  value: T,
  forbidden: T,
  code: string,
  message: string | null,
  path: Path,
): ValidationReport {
  return value !== forbidden ? ValidationReport.ok() : reportOne(path, code, message)
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
