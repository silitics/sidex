declare const SIDEX_PATH_SYMBOL: unique symbol

/**
 * A nominal type based on `T` and identified by it's Sidex path `P`.
 */
export type Nominal<T, P extends string> = T & { [SIDEX_PATH_SYMBOL]: P }

export default Nominal
