# Sidex: Values

Efficient dynamically-typed *values*.

Values can be:

- Serialized.
- Accessed using paths.
    + `get`: Retrieves a shared reference to a sub value by path.
    + `get_mut`: Retrieves a mutable reference to a sub value by path.
    + `take`: Removes a sub value by path and returns it.
    + `swap`: Swaps two sub values by path.
    + `set`: Sets a value from a boxed value.
    + `set_deserialized`: Sets the value by deserializing it from a deserializer.
    + `insert`: Inserts a boxed value by path.
    + `insert_deserialized`: Inserts a value by deserializing it from a deserializer.