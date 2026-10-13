/** Structural checks for Sidex's JSON representation. */
import * as builtins from "./builtins.js"

/** A segment in a JSON path. */
export type PathSegment =
  | { readonly kind: "key"; readonly key: string }
  | { readonly kind: "index"; readonly index: number }

/** A JSON path, formatted like Rust decode paths and validation paths. */
export class Path {
  private constructor(readonly segments: readonly PathSegment[]) {}
  static root(): Path {
    return new Path([])
  }
  key(key: string): Path {
    return new Path([...this.segments, { kind: "key", key }])
  }
  field(name: string): Path {
    return this.key(name)
  }
  index(index: number): Path {
    return new Path([...this.segments, { kind: "index", index }])
  }
  isRoot(): boolean {
    return this.segments.length === 0
  }
  toString(): string {
    return this.segments
      .map((segment, index) =>
        segment.kind === "index"
          ? `[${segment.index}]`
          : `${index === 0 ? "" : "."}${segment.key}`,
      )
      .join("")
  }
}

/** One structural error, compatible with a validation error. */
export interface CheckError {
  readonly path: string
  readonly code: "type" | "missing" | "case" | "key"
  readonly message: string
  /** Unambiguous path segments, including keys containing dots or brackets. */
  readonly segments: readonly PathSegment[]
}

/** All structural errors. An empty report means the representation is valid. */
export class CheckReport {
  constructor(private readonly failures: CheckError[] = []) {}
  get errors(): readonly CheckError[] {
    return this.failures
  }
  isOk(): boolean {
    return this.failures.length === 0
  }
  isErr(): boolean {
    return !this.isOk()
  }
  push(error: CheckError): void {
    this.failures.push(error)
  }
  merge(report: CheckReport): void {
    for (const error of report.errors) this.failures.push(error)
  }
  toString(): string {
    return this.failures
      .map((error) => `${error.code} at ${error.path}: ${error.message}`)
      .join("\n")
  }
}

/** Runtime evidence for a type, also used as an argument to generic checks. */
export interface Checker<T> {
  /** Whether the Sidex payload has a record shape (including aliases and wrappers). */
  isRecord(): boolean
  is(value: unknown): value is T
  check(value: unknown, path?: Path): CheckReport
  /** Used by generated checks to collect errors and decode map keys. */
  checkAt(value: unknown, path: Path, report: CheckReport, key?: boolean): void
}

/** A structural check that appends errors to a report. */
export type Check = ((
  value: unknown,
  path: Path,
  report: CheckReport,
  key: boolean,
) => void) & { isRecord?: () => boolean }

/** Constructs a checker lazily, allowing recursive and cross-schema references. */
export function checker<T>(resolve: () => Check): Checker<T> {
  let resolved: Check | undefined
  const checkAt: Checker<T>["checkAt"] = (value, path, report, key = false) => {
    resolved ??= resolve()
    if (value === null || typeof value !== "object") {
      resolved(value, path, report, key)
      return
    }
    const ancestors = active.get(report) ?? new Map<object, number>()
    active.set(report, ancestors)
    const previous = ancestors.get(value)
    const depth = path.segments.length
    if (previous !== undefined) {
      if (depth > previous)
        return fail(report, path, "type", "Cyclic values are not JSON.")
      resolved(value, path, report, key)
      return
    }
    ancestors.set(value, depth)
    try {
      resolved(value, path, report, key)
    } finally {
      ancestors.delete(value)
    }
  }
  const check = (value: unknown, path = Path.root()) => {
    const report = new CheckReport()
    checkAt(value, path, report)
    return report
  }
  const isRecord = () => {
    resolved ??= resolve()
    return resolved.isRecord?.() ?? false
  }
  return {
    checkAt,
    check,
    isRecord,
    is: (value: unknown): value is T => check(value).isOk(),
  }
}

/** Defers references until the check runs, avoiding module initialization cycles. */
export function lazy(resolve: () => Checker<unknown>): Check {
  return Object.assign(
    (value: unknown, path: Path, report: CheckReport, key: boolean) =>
      resolve().checkAt(value, path, report, key),
    { isRecord: () => resolve().isRecord() },
  )
}

/** Checks one scalar using its existing built-in guard. */
export function scalar(
  expected: string,
  is: (value: unknown) => boolean,
  isKey = is,
): Check {
  return (value, path, report, key) => {
    if (!(key ? isKey(value) : is(value)))
      fail(report, path, key ? "key" : "type", `Expected ${expected}.`)
  }
}

/** Checks fields and inline payloads while ignoring unknown record fields. */
export function record(
  fields: readonly (readonly [string, Check, boolean])[],
  inline: readonly Check[] = [],
): Check {
  const check: Check = (value, path, report, key) => {
    if (key || !isObject(value))
      return fail(report, path, "type", "Expected an object.")
    for (const [name, check, optional] of fields) {
      const present = hasOwn(value, name)
      if (optional && (!present || value[name] === null)) continue
      if (!present)
        fail(report, path.key(name), "missing", `Missing field '${name}'.`)
      else check(value[name], path.key(name), report, false)
    }
    for (const check of inline) check(value, path, report, false)
  }
  return Object.assign(check, { isRecord: () => true })
}

/** Checks every sequence element. */
export function sequence(element: Check): Check {
  return (value, path, report, key) => {
    if (key || !Array.isArray(value))
      return fail(report, path, "type", "Expected an array.")
    for (let index = 0; index < value.length; index++)
      element(value[index], path.index(index), report, false)
  }
}

/** Checks both the text key and the value of every map entry. */
export function map(keyCheck: Check, valueCheck: Check): Check {
  return (value, path, report, key) => {
    if (key || !isObject(value))
      return fail(report, path, "type", "Expected an object.")
    for (const [name, entry] of Object.entries(value)) {
      keyCheck(name, path.key(name), report, true)
      valueCheck(entry, path.key(name), report, false)
    }
  }
}

/** Case name, payload check, and content field (null means a shared object). */
export type Case = readonly [
  string,
  Check | null,
  string | null | readonly [string],
]

/** Checks all Sidex tagging modes. Failed implicit cases keep their errors. */
export function variant(
  mode: "external" | "internal" | "adjacent" | "implicit",
  tag: string,
  cases: readonly Case[],
): Check {
  return (value, path, report, key) => {
    if (key) return fail(report, path, "key", "Variants cannot be map keys.")
    if (mode === "implicit") {
      const failures = new CheckReport()
      for (const [name, payload] of cases) {
        if (payload === null) {
          if (value === name) return
          continue
        }
        const attempt = new CheckReport()
        if (active.has(report)) active.set(attempt, active.get(report)!)
        payload(value, path, attempt, false)
        if (attempt.isOk()) return
        failures.merge(attempt)
      }
      fail(report, path, "case", "No variant case matches.")
      report.merge(failures)
      return
    }
    if (mode === "external") {
      if (typeof value === "string") {
        if (cases.some(([name, payload]) => name === value && payload === null))
          return
        return fail(report, path, "case", "Expected a case without a payload.")
      }
      if (!isObject(value) || Object.keys(value).length !== 1)
        return fail(
          report,
          path,
          "type",
          "Expected a case name or an object with one entry.",
        )
      const name = Object.keys(value)[0]!
      const selected = cases.find(([caseName]) => caseName === name)
      if (!selected || selected[1] === null)
        return fail(report, path, "case", "Expected a case with a payload.")
      selected[1](value[name], path.key(name), report, false)
      return
    }
    if (!isObject(value))
      return fail(report, path, "type", "Expected an object.")
    if (!hasOwn(value, tag))
      return fail(report, path.key(tag), "missing", `Missing field '${tag}'.`)
    const selected = cases.find(([name]) => name === value[tag])
    if (!selected)
      return fail(
        report,
        path.key(tag),
        "case",
        "Expected a known variant tag.",
      )
    const [, payload, contentSpec] = selected
    if (payload === null) return
    const content =
      typeof contentSpec === "object" && contentSpec !== null
        ? payload.isRecord?.()
          ? null
          : contentSpec[0]
        : contentSpec
    if (content === null) payload(value, path, report, false)
    else if (!hasOwn(value, content))
      fail(report, path.key(content), "missing", `Missing field '${content}'.`)
    else payload(value[content], path.key(content), report, false)
  }
}

/** Checks an opaque JSON shape, including JSON-only values within containers. */
export function jsonShape(shapes: readonly string[]): Check {
  return scalar(shapes.join(" or "), (value) => {
    const kind =
      value === null ? "null" : Array.isArray(value) ? "array" : typeof value
    return (
      (shapes.includes("any") || shapes.includes(kind)) &&
      builtins.Json.is(value)
    )
  })
}

/** Checks a built-in's wire representation independently of type mappings. */
export function builtin(name: string): Check {
  if (name === "json") return jsonShape(["any"])
  const guard = scalarGuards[name]!
  if (["bool", "f32", "f64"].includes(name))
    return scalar(name, guard, () => false)
  const range = integerRanges[name]
  return scalar(
    name,
    guard,
    range ? (value) => integerKey(value, ...range) : guard,
  )
}

const active = new WeakMap<CheckReport, Map<object, number>>()

const scalarGuards: Record<string, (value: unknown) => boolean> = {
  string: builtins.String.is,
  bool: builtins.Bool.is,
  i8: builtins.I8.is,
  i16: builtins.I16.is,
  i32: builtins.I32.is,
  i64: builtins.I64.is,
  u8: builtins.U8.is,
  u16: builtins.U16.is,
  u32: builtins.U32.is,
  u64: builtins.U64.is,
  idx: builtins.Idx.is,
  f32: builtins.F32.is,
  f64: builtins.F64.is,
  bytes: builtins.Bytes.is,
  uuid: builtins.Uuid.is,
  timestamp: builtins.Timestamp.is,
  date: builtins.Date.is,
}
const integerRanges: Record<string, readonly [bigint, bigint]> = {
  i8: [-128n, 127n],
  i16: [-32768n, 32767n],
  i32: [-2147483648n, 2147483647n],
  i64: [-(2n ** 63n), 2n ** 63n - 1n],
  u8: [0n, 255n],
  u16: [0n, 65535n],
  u32: [0n, 4294967295n],
  u64: [0n, 2n ** 64n - 1n],
  idx: [0n, 2n ** 64n - 1n],
}
function integerKey(value: unknown, min: bigint, max: bigint): boolean {
  if (
    typeof value !== "string" ||
    !(min < 0n ? /^[+-]?[0-9]+$/ : /^\+?[0-9]+$/).test(value)
  )
    return false
  const integer = BigInt(value)
  return min <= integer && integer <= max
}
function hasOwn(value: object, name: string): boolean {
  return Object.prototype.hasOwnProperty.call(value, name)
}
function isObject(value: unknown): value is Record<string, unknown> {
  return (
    value !== null &&
    typeof value === "object" &&
    !Array.isArray(value) &&
    (Object.getPrototypeOf(value) === Object.prototype ||
      Object.getPrototypeOf(value) === null)
  )
}
function fail(
  report: CheckReport,
  path: Path,
  code: CheckError["code"],
  message: string,
): void {
  report.push({ path: path.toString(), segments: path.segments, code, message })
}
