/**
 * Unique symbol used for nominal types.
 */
declare const SIDEX_PATH_SYMBOL: unique symbol

// This type improves error messages.
type N<P extends string> ={ [SIDEX_PATH_SYMBOL]: { [key in P]: null } }

/**
 * A nominal type based on `T` and identified by it's Sidex path `P`.
 */
export type Nominal<T, P extends string> = T & N<P>

/**
 * The structural type of a nominal type `T`.
 */
export type Structural<T> = T extends undefined
  ? undefined
  : T extends boolean
  ? boolean
  : T extends null
  ? null
  : T extends string
  ? string
  : T extends number
  ? number
  : T extends boolean
  ? boolean
  : T extends Array<infer U>
  ? U[]
  : T extends (...args: infer Args) => infer Return
  ? (...args: Args) => Return
  : Omit<T, typeof SIDEX_PATH_SYMBOL>

export default Nominal
