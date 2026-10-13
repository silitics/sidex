/**
 * Types of Sidex's built-in types in their JSON representation.
 *
 * Values of these types are what `JSON.parse` returns for data produced by any
 * Sidex implementation. 64-bit integers beyond `Number.MAX_SAFE_INTEGER` are
 * decimal strings, because JavaScript numbers cannot represent them exactly.
 * Non-finite floats are the strings
 * `"NaN"`, `"+Infinity"`, and `"-Infinity"`, because JSON numbers cannot
 * represent them. The companion objects convert between these representations
 * and JavaScript values.
 *
 * @module
 */

import type { Nominal } from "./nominal.js"
import { isDate, isTimestamp } from "./temporal.js"

/**
 * A string.
 *
 * We do not need `Nominal` here because any `string` is a valid Sidex string.
 */
export type String = string

export const String = {
  /** Checks a JSON string without coercion. */
  is(value: unknown): value is String {
    return typeof value === "string"
  },
}

/**
 * A boolean.
 */
export type Bool = boolean

export const Bool = {
  /** Checks a JSON boolean without coercion. */
  is(value: unknown): value is Bool {
    return typeof value === "boolean"
  },
}

// ---- Small integers --------------------------------------------------------

/**
 * An 8-bit signed integer.
 */
export type I8 = Nominal<number, "::core::builtins::i8">

export const I8 = smallInt<I8>(-128, 127)

/**
 * A 16-bit signed integer.
 */
export type I16 = Nominal<number, "::core::builtins::i16">

export const I16 = smallInt<I16>(-32_768, 32_767)

/**
 * A 32-bit signed integer.
 */
export type I32 = Nominal<number, "::core::builtins::i32">

export const I32 = smallInt<I32>(-2_147_483_648, 2_147_483_647)

/**
 * An 8-bit unsigned integer.
 */
export type U8 = Nominal<number, "::core::builtins::u8">

export const U8 = smallInt<U8>(0, 255)

/**
 * A 16-bit unsigned integer.
 */
export type U16 = Nominal<number, "::core::builtins::u16">

export const U16 = smallInt<U16>(0, 65_535)

/**
 * A 32-bit unsigned integer.
 */
export type U32 = Nominal<number, "::core::builtins::u32">

export const U32 = smallInt<U32>(0, 4_294_967_295)

/**
 * An index into a sequence on a 64-bit target. Numbers beyond 2^53 lose precision.
 */
export type Idx = Nominal<number, "::core::builtins::idx">

export const Idx = smallInt<Idx>(0, 18_446_744_073_709_549_568)

/**
 * Operations of an integer type represented as a JavaScript number.
 */
export interface SmallInt<T extends number> {
  /** The smallest value. */
  readonly MIN: T
  /** The largest value. */
  readonly MAX: T
  /** Checks whether the value is an integer in the range of the type. */
  is(value: unknown): value is T
  /**
   * Converts a number to the type.
   *
   * @throws {RangeError} If the number is not an integer in the range.
   */
  fromNumber(value: number): T
}

function smallInt<T extends number>(min: number, max: number): SmallInt<T> {
  const is = (value: unknown): value is T =>
    typeof value === "number" &&
    Number.isInteger(value) &&
    value >= min &&
    value <= max
  return {
    MIN: min as T,
    MAX: max as T,
    is,
    fromNumber(value: number): T {
      if (!is(value)) {
        throw new RangeError(`${value} is not an integer in [${min}, ${max}]`)
      }
      return value
    },
  }
}

// ---- 64-bit integers -------------------------------------------------------

// Declared before `I64` and `U64`, whose construction reads it.
const MAX_SAFE = BigInt(Number.MAX_SAFE_INTEGER)

/**
 * A 64-bit signed integer, represented as a number or, if its magnitude exceeds
 * `Number.MAX_SAFE_INTEGER`, as a decimal string.
 */
export type I64 = Nominal<number | string, "::core::builtins::i64">

export const I64 = bigInt<I64>(-(2n ** 63n), 2n ** 63n - 1n)

/**
 * A 64-bit unsigned integer, represented as a number or, if it exceeds
 * `Number.MAX_SAFE_INTEGER`, as a decimal string.
 */
export type U64 = Nominal<number | string, "::core::builtins::u64">

export const U64 = bigInt<U64>(0n, 2n ** 64n - 1n)

/**
 * Operations of a 64-bit integer type.
 */
export interface BigInt64<T extends number | string> {
  /** The smallest value. */
  readonly MIN: T
  /** The largest value. */
  readonly MAX: T
  /**
   * Checks whether the value is an integer number or a decimal string in the
   * range of the type.
   */
  is(value: unknown): value is T
  /**
   * Converts a `bigint` to the type.
   *
   * @throws {RangeError} If the value is out of range.
   */
  fromBigInt(value: bigint): T
  /** Converts a value of the type to a `bigint`. This is exact. */
  toBigInt(value: T): bigint
  /**
   * Converts a number to the type.
   *
   * @throws {RangeError} If the number is not a safe integer in the range.
   */
  fromNumber(value: number): T
  /**
   * Converts a value of the type to the nearest number.
   *
   * Values beyond `Number.MAX_SAFE_INTEGER` lose precision. Use
   * {@link BigInt64.toBigInt} for exact values.
   */
  toNumber(value: T): number
}

function bigInt<T extends number | string>(
  min: bigint,
  max: bigint,
): BigInt64<T> {
  const inRange = (value: bigint) => value >= min && value <= max
  const fromBigInt = (value: bigint): T => {
    if (!inRange(value)) {
      throw new RangeError(`${value} is not in [${min}, ${max}]`)
    }
    return (
      -MAX_SAFE <= value && value <= MAX_SAFE ? Number(value) : value.toString()
    ) as T
  }
  return {
    MIN: fromBigInt(min),
    MAX: fromBigInt(max),
    is(value: unknown): value is T {
      if (typeof value === "number") {
        return Number.isInteger(value) && inRange(BigInt(value))
      }
      return (
        typeof value === "string" &&
        (min < 0n ? /^[+-]?[0-9]+$/ : /^\+?[0-9]+$/).test(value) &&
        inRange(BigInt(value))
      )
    },
    fromBigInt,
    toBigInt(value: T): bigint {
      return BigInt(value)
    },
    fromNumber(value: number): T {
      if (!Number.isSafeInteger(value)) {
        throw new RangeError(`${value} is not a safe integer`)
      }
      return fromBigInt(BigInt(value))
    },
    toNumber(value: T): number {
      return Number(value)
    },
  }
}

// ---- Floats ----------------------------------------------------------------

/**
 * Representations of non-finite floats. Writers use `"+Infinity"`, and
 * readers also accept `"Infinity"`.
 */
export type NonFinite = "NaN" | "+Infinity" | "Infinity" | "-Infinity"

/**
 * A 32-bit float, represented as a number or, if it is not finite, as a string.
 */
export type F32 = Nominal<number | NonFinite, "::core::builtins::f32">

export const F32 = float<F32>()

/**
 * A 64-bit float, represented as a number or, if it is not finite, as a string.
 */
export type F64 = Nominal<number | NonFinite, "::core::builtins::f64">

export const F64 = float<F64>()

/**
 * Operations of a float type.
 */
export interface Float<T> {
  /** Checks whether the value is a finite number or an accepted non-finite string. */
  is(value: unknown): value is T
  /** Converts a number to the type. */
  fromNumber(value: number): T
  /** Converts a value of the type to a number. */
  toNumber(value: T): number
}

function float<T>(): Float<T> {
  return {
    is(value: unknown): value is T {
      return (
        (typeof value === "number" && Number.isFinite(value)) ||
        value === "NaN" ||
        value === "Infinity" ||
        value === "+Infinity" ||
        value === "-Infinity"
      )
    },
    fromNumber(value: number): T {
      if (Number.isFinite(value)) return value as T
      if (Number.isNaN(value)) return "NaN" as T
      return (value > 0 ? "+Infinity" : "-Infinity") as T
    },
    toNumber(value: T): number {
      if (typeof value === "number") return value
      switch (value as string) {
        case "NaN":
          return NaN
        case "Infinity":
        case "+Infinity":
          return Infinity
        case "-Infinity":
          return -Infinity
        default:
          throw new TypeError(`${value} is not a float`)
      }
    },
  }
}

// ---- Bytes -----------------------------------------------------------------

/**
 * A sequence of bytes, represented as a Base64 string with padding.
 */
export type Bytes = Nominal<string, "::core::builtins::bytes">

export const Bytes = {
  /** Checks standard Base64, with optional padding and zero trailing bits. */
  is(value: unknown): value is Bytes {
    if (
      typeof value !== "string" ||
      !/^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}={0,2}|[A-Za-z0-9+/]{3}=?)?$/.test(
        value,
      )
    )
      return false
    const text = value.replace(/=+$/, "")
    const remainder = text.length % 4
    const digit =
      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".indexOf(
        text[text.length - 1] ?? "A",
      )
    return (
      remainder === 0 ||
      (remainder === 2 ? (digit & 15) === 0 : (digit & 3) === 0)
    )
  },
  /** Encodes bytes. */
  fromUint8Array(bytes: Uint8Array): Bytes {
    let binary = ""
    for (const byte of bytes) binary += globalThis.String.fromCharCode(byte)
    return btoa(binary) as Bytes
  },
  /** Decodes bytes. */
  toUint8Array(value: Bytes): Uint8Array {
    if (!Bytes.is(value)) throw new TypeError(`${value} is not Base64`)
    const binary = atob(value.replace(/=+$/, ""))
    const bytes = new Uint8Array(binary.length)
    for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i)
    return bytes
  },
}

// ---- Identifiers and time --------------------------------------------------

/**
 * A UUID, represented as a lowercase hyphenated string.
 */
export type Uuid = Nominal<string, "::core::builtins::uuid">

export const Uuid = {
  /** Checks whether the value is a hyphenated UUID string. */
  is(value: unknown): value is Uuid {
    return (
      typeof value === "string" &&
      /^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/.test(
        value,
      )
    )
  },
  /**
   * Normalizes a hyphenated UUID string.
   *
   * @throws {TypeError} If the string is not a hyphenated UUID.
   */
  parse(value: string): Uuid {
    if (!Uuid.is(value)) throw new TypeError(`${value} is not a UUID`)
    return value.toLowerCase() as Uuid
  },
}

/**
 * An instant in time with nanosecond precision, represented as an RFC 3339
 * string in UTC.
 */
export type Timestamp = Nominal<string, "::core::builtins::timestamp">

export const Timestamp = {
  /** Checks a timestamp accepted by the Rust JSON reader. */
  is(value: unknown): value is Timestamp {
    return isTimestamp(value)
  },
  /** Converts a JavaScript date. */
  fromDate(date: globalThis.Date): Timestamp {
    return date.toISOString() as Timestamp
  },
  /** Converts to a JavaScript date, which has millisecond precision. */
  toDate(value: Timestamp): globalThis.Date {
    return new globalThis.Date(value)
  },
  /** Returns the current time. */
  now(): Timestamp {
    return new globalThis.Date().toISOString() as Timestamp
  },
}

/**
 * A date in the Gregorian calendar, represented as `YYYY-MM-DD`.
 */
export type Date = Nominal<string, "::core::builtins::date">

export const Date = {
  /** Checks a valid calendar date accepted by the Rust JSON reader. */
  is(value: unknown): value is Date {
    return isDate(value)
  },
}

// ---- Compound types --------------------------------------------------------

/**
 * A JSON value.
 */
export type Json =
  null | boolean | number | string | Json[] | { [key: string]: Json }

export const Json = {
  /** Checks JSON values, rejecting cycles and JavaScript-only values. */
  is(value: unknown): value is Json {
    return isJsonValue(value)
  },
}

/**
 * A sequence.
 */
export type Sequence<T> = T[]

/**
 * The text representation of a map key.
 *
 * Integer keys are decimal strings; string key brands retain their identity.
 */
export type MapKey<K> = K extends number ? `${K}` : K extends string ? K : never

/** A JSON map with optional entries for finite key types. */
export type ObjectMap<K, V> = string extends K
  ? { [key: string]: V }
  : { [key in MapKey<K>]?: V }

/**
 * Returns the entries of a map.
 *
 * Unlike `Object.entries`, the entries keep the map's key type, such as a
 * nominal identifier type, and omit absent keys.
 */
export function entries<K extends string, V>(
  map: ObjectMap<K, V>,
): [K, Exclude<V, undefined>][] {
  return (Object.entries(map) as [K, V][]).filter(
    (entry): entry is [K, Exclude<V, undefined>] => entry[1] !== undefined,
  )
}

/** Checks containers without requiring a JSON serialization round trip. */
function isJsonValue(
  value: unknown,
  ancestors = new Set<object>(),
): value is Json {
  if (value === null || typeof value === "string" || typeof value === "boolean")
    return true
  if (typeof value === "number") return Number.isFinite(value)
  if (
    typeof value !== "object" ||
    (!Array.isArray(value) &&
      Object.getPrototypeOf(value) !== Object.prototype &&
      Object.getPrototypeOf(value) !== null)
  )
    return false
  if (ancestors.has(value)) return false
  ancestors.add(value)
  const valid = (
    Array.isArray(value) ? Array.from(value) : Object.values(value)
  ).every((item) => isJsonValue(item, ancestors))
  ancestors.delete(value)
  return valid
}
