/**
 * Unique symbol used for nominal types.
 */
declare const SIDEX_PATH_SYMBOL: unique symbol

/**
 * A nominal type based on `T` and identified by it's Sidex path `P`.
 */
export type Nominal<T, P extends string> = T & { [SIDEX_PATH_SYMBOL]: P }

/**
 * The structural type of a nominal type `T`.
 */
export type Structural<T> = T extends string
  ? string
  : T extends number
  ? number
  : T extends boolean
  ? boolean
  : T extends any[]
  ? T[number][]
  : T extends (...args: infer Args) => infer Return
  ? (...args: Args) => Return
  : Omit<T, typeof SIDEX_PATH_SYMBOL>

export default Nominal