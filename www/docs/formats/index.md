# Interchange Formats

Almost everything in engineering is a tradeoff and _data interchange formats_ are no exception to this rule. Sidex is _format-agnostic_ meaning that you can choose the interchange format which fits your specific needs best. To this end, a _format mapping_ specifies how Sidex types are represented with the primitives and structures provided by an interchange format.

:::note

Of course, without specifying a format mapping and writing a code generator yourself, you can only choose a format for which a mapping has been specified and which is supported by the generators for the programming languages you are using.

:::

This part of the documentation standardizes format mappings from Sidex types to interchange formats.

## Half-Baked Thoughts

The following thoughts regarding format mappings are only half-baked but may still be of value to some.

### Relation to Serde's `Serializer` Trait

[Serde](https:://serde.rs) defines the [`Serializer`](https://docs.rs/serde/latest/serde/trait.Serializer.html) trait which can be implemented for different interchange formats. In some sense, this trait provides an abstraction for specifying a format mapping independently[^1] of the actual format:

> “_The Serde ecosystem consists of data structures that know how to serialize and deserialize themselves along with data formats that know how to serialize and deserialize other things. Serde provides the layer by which these two groups interact with each other, allowing any supported data structure to be serialized and deserialized using any supported data format._”
>
> — [https:://serde.rs](https:://serde.rs)

The way this leverages Rust's trait system is an engineering marvel and Serde ows its huge success to precisely this decoupling. The only disadvantage of this approach is that data structures are, in general, limited to the methods provided by the `Serializer` trait. This necessitates some tricks when trying to use specific features of a serialization format. For instance, the [BSON](https://crates.io/crates/bson) implementation for Serde [uses special names for structs](https://github.com/mongodb/bson-rust/blob/0612667e1344f9aabc3592a2cee02a96ac1b76bc/src/ser/serde.rs#L306) in order to control the BSON serialization of some types.

Note that the same thoughts also apply to deserialization with Serde.

In the long run and especially for the Rust backend of Sidex, a similar approach is advisable.

As we see it, this approach does not alleviate the need for format mappings but instead provides a way of specifying format mappings independently of any concrete format by utilizing clever abstractions. In some sense, Sidex works in a similar way for record, variant, and wrapper types which are mapped (to a language and format) in an automated way.

[^1]: Note that there is a method [`is_human_readable`](https://docs.rs/serde/latest/serde/trait.Serializer.html#method.is_human_readable) which can be used to query whether the interchange format is human-readable and choose a different mapping (e.g., string or bytes as for [`Uuid`](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#impl-Serialize-for-Uuid)) based on whether it is.
