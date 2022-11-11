---
sidebar_position: 6
---

# Exchanging Data

**🚧 TODO**: This section should be rewritten from a user's perspective and the current content moved into the _Advanced Topics_.

Data exchange can be quite complex and involves multiple concerns which Sidex aims to separate. In particular, there need to be various mappings from Sidex type and service definitions to representations in different languages and interchange formats.

## Language Mappings

To be useful, Sidex type and service definitions need to be _mapped_ to type or class definitions of some programming language, e.g., [Rust](https://rust-lang.org) or [TypeScript](https://www.typescriptlang.org). We refer to such a mapping as a _language mapping_:

```plain
┌───────────────────┐   Language Mapping    ┌─────────────────┐
│ Sidex Definitions │ ────────────────────► │ Target Language │
└───────────────────┘                       └─────────────────┘
```

Note that a mapping from Sidex definitions to a language may involve certain trade-offs. For instance, in case of TypeScript, a map type `[K: V]` can be mapped either to `Object` or to `Map`, and, in case of Rust, there are also multiple different types of maps available, e.g., `HashMap` or `BTreeMap`. Furthermore, depending on the language, certain data types may not be mappable at all due to language-specific constraints.
The goal of Sidex is to provide tools and infrastructure for mapping Sidex definitions to different programming languages _without_ imposing any particular mapping. Using the [`sidex-gen`](https://crates.io/crates/sidex-gen) crate as a basis, you can define your own mappings and even generate additional boilerplate such as constructors and getters. If something cannot be sensibly mapped, a tool is free to generate an error as a last resort.
For officially supported languages, out-of-the-box mappings with sane defaults are provided.

Note that a language mapping is itself completely independent from how data may be serialized or how methods on a service may be invoked via a protocol. It can also be useful without ever exchanging any data.

For user-defined opaque types, specific language mappings have to be provided.

## Format Mappings

To exchange data between different languages or processes, it needs to be _serialized_ into some common _interchange format_. To this end, a _format mapping_ from a Sidex definitions to the interchange format is necessary:

```plain
┌───────────────────┐   Format Mapping    ┌─────────────────────┐
│ Sidex Definitions │ ──────────────────► │ Interchange Formats │
└───────────────────┘                     └─────────────────────┘
```

Note that the format mapping is supposed to be language-independent. It merely describes how certain Sidex types are mapped to the interchange format and its types. Again, Sidex does not impose any restrictions on the interchange format, however, it aims to provide some standardized mappings to common formats such as JSON.

For user-defined opaque types, specific format mappings have to be provided.

## Serialization Binding

Once we have fixed a language mapping and a format mapping, we need to bind both together. To this end, code generators generates a _serialization binding_ which is language-specific and format-specific. It takes serialized data as per the format mapping and transforms it into data structures as per the language mapping (known as _deserialization_) and vice versa (known as _serialization_) bridging the gap between an interchange format and native data structures.
