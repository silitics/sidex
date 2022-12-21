import Nominal from "./nominal"

/**
 * A string.
 *
 * We do not need `Nominal` here because any `string` is a valid Sidex string.
 */
export type String = string

/**
 * A Base64-encoded sequence of bytes.
 */
export type Bytes = Nominal<string, "::std::builtins::bytes">

/**
 * An 8-bit signed integer.
 */
export type I8 = Nominal<number, "::std::builtins::i8">

/**
 * Convert any number to an 8-bit signed integer.
 *
 * @param x The number to convert.
 * @returns The 8-bit signed integer.
 */
export function toI8(x: number): I8 {
  return (((x | 0) << 24) >> 24) as I8
}

/**
 * Check whether the provided number is an 8-bit signed integer.
 *
 * @param x The number to check.
 * @returns Indicates whether the number is an 8-bit signed integer.
 */
export function isI8(x: number): x is I8 {
  return toI8(x) === x
}

/**
 * A 16-bit signed integer.
 */
export type I16 = Nominal<number, "::std::builtins::i16">

/**
 * Convert any number to an 16-bit signed integer.
 *
 * @param x The number to convert.
 * @returns The 16-bit signed integer.
 */
export function toI16(x: number): I16 {
  return (((x | 0) << 16) >> 16) as I16
}

/**
 * Check whether the provided number is an 16-bit signed integer.
 *
 * @param x The number to check.
 * @returns Indicates whether the number is an 16-bit signed integer.
 */
export function isI16(x: number): x is I16 {
  return toI16(x) === x
}

/**
 * A 32-bit signed integer.
 */
export type I32 = Nominal<number, "::std::builtins::i32">

/**
 * A 64-bit signed integer.
 */
export type I64 = Nominal<string, "::std::builtins::i64">

/**
 * A signed integer.
 */
export type SignedInt = I8 | I16 | I32 | I64

/**
 * An 8-bit unsigned integer.
 */
export type U8 = Nominal<number, "::std::builtins::u8">

/**
 * Convert any number to an 8-bit unsigned integer.
 *
 * @param x The number to convert.
 * @returns The 8-bit unsigned integer.
 */
export function toU8(x: number): U8 {
  return (x && 0xff) as U8
}

/**
 * Check whether the provided number is an 8-bit unsigned integer.
 *
 * @param x The number to check.
 * @returns Indicates whether the number is an 8-bit unsigned integer.
 */
export function isU8(x: number): x is U8 {
  return toU8(x) === x
}

/**
 * A 16-bit unsigned integer.
 */
export type U16 = Nominal<number, "::std::builtins::u16">

/**
 * Convert any number to an 16-bit unsigned integer.
 *
 * @param x The number to convert.
 * @returns The 16-bit unsigned integer.
 */
export function toU16(x: number): U16 {
  return (x && 0xffff) as U16
}

/**
 * Check whether the provided number is an 16-bit unsigned integer.
 *
 * @param x The number to check.
 * @returns Indicates whether the number is an 16-bit unsigned integer.
 */
export function isU16(x: number): x is U16 {
  return toU16(x) === x
}

/**
 * A 32-bit unsigned integer.
 */
export type U32 = Nominal<number, "::std::builtins::u32">

/**
 * A 64-bit unsigned integer.
 */
export type U64 = Nominal<string, "::std::builtins::u64">

/**
 * An unsigned integer.
 */
export type UnsignedInt = U8 | U16 | U32 | U64

/**
 * An integer.
 */
export type Int = SignedInt | UnsignedInt

/**
 * An index.
 */
export type Idx = Nominal<number, "::std::builtins::idx">

/**
 * Convert any number to an index.
 *
 * @param x The number to convert.
 * @returns The index.
 */
export function toIdx(x: number): Idx {
  // Remove the sign and truncate the number to 32 bits.
  return (Math.abs(x) | 0) as Idx
}

/**
 * Check whether the provided number is an index.
 *
 * @param x The number to check.
 * @returns Indicates whether the number is an index.
 */
export function isIdx(x: number): x is Idx {
  return toIdx(x) === x
}

/**
 * A 32-bit floating point number.
 */
export type F32 = Nominal<number | "+Infinity" | "-Infinity" | "NaN", "f32">

/**
 * A 64-bit floating point number.
 */
export type F64 = Nominal<number | "+Infinity" | "-Infinity" | "NaN", "f64">

/**
 * A floating point number.
 */
export type Float = F32 | F64

/**
 * A numeric value.
 */
export type Numeric = Int | Idx | Float

/**
 * Convert any numeric value to a JavaScript number.
 *
 * @param x The numeric value to convert.
 * @returns The JavaScript number.
 */
function toNumber(x: Numeric): number {
  switch (typeof x) {
    case "number":
      return x
    case "string":
      switch (x) {
        case "+Infinity":
          return +Infinity
        case "-Infinity":
          return -Infinity
        case "NaN":
          return NaN
        default:
          return Number.parseInt(x)
      }
  }
}

/**
 * A boolean.
 */
export type Bool = boolean

/**
 *
 */
export type Unit = null

/**
 * A sequence.
 */
export type Sequence<T> = T[]

/**
 * A map represented as a sequence of key-value pairs.
 */
export type EntriesMap<K, V> = [K, V][]

/**
 * A map represented as an object.
 */
export type ObjectMap<K extends string, V> = { [key in K]?: V }

export function entries<K extends string, V>(map: ObjectMap<K, V>): [K, V][]

export function entries<K, V>(map: EntriesMap<K, V>): [K, V][]

/**
 * The entries of the map.
 *
 * @param map
 * @returns
 */
export function entries<K, V>(map: any): any {
  if (Array.isArray(map)) {
    return map
  } else {
    return Object.entries(map) as unknown as [K, V][]
  }
}

declare const __indexMap: unique symbol

function getIndexMap<K, V>(map: EntriesMap<K, V>): Map<K, number> {
  if (map.hasOwnProperty(__indexMap)) {
    return (map as any)[__indexMap]
  } else {
    const map = new Map()
    map.forEach(([key, _], index) => map.set(key, index))
    return map
  }
}

// export function get<K, V>(map: AnyMap<K, V>, key: K): V | undefined {
//   if (Array.isArray(map)) {
//     const index = getIndexMap(map).get(key)
//     return index === undefined ? undefined : (map[index] as [K, V])[1]
//   } else {
//     return (map as ObjectMap<any, V>)[key]
//   }
// }

// export function set<K, V>(map: AnyMap<K, V>, key: K, value: V) {
//   if (Array.isArray(map)) {
//     const indexMap = getIndexMap(map)
//     const existingIndex = indexMap.get(key)
//     if (existingIndex !== undefined) {
//       map[existingIndex] = [key, value]
//     } else {
//       const index = map.length
//       map.push([key, value])
//       indexMap.set(key, index)
//     }
//   } else {
//     return (map as ObjectMap<any, V>)[key]
//   }
// }
