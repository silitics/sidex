---
title: API Groups
description: Document which definitions share a compatibility promise.
order: 4
---

A bundle often serves consumers that update on different schedules. For example, a web app may deploy together with its server, while installed mobile apps keep using old releases for months. Changing a request that only the web app uses is cheap. Changing a request that mobile apps use can break consumers you cannot update.

A _group_ names a set of definitions and describes a promise that their consumers rely on, such as how long changes must stay backward compatible. Sidex records which definitions belong to each group and adds the group's description to their generated documentation. Developers reading a schema or generated code can then see which promises a change must keep. [Lifecycle attributes](/sidex/docs/guides/api-lifecycle/) describe where an item is in its lifecycle; groups describe who depends on it.

Groups only document promises; Sidex does not check changes against them.

## Declare a Group

Declare groups in the bundle's `sidex.toml`:

```toml
[groups.mobile]
description = "Used by released mobile apps. Keep changes backward compatible for at least 12 months."
```

The table key is the group name. The `description` is required. Generated documentation repeats it on every member after a bold `Group` label, so keep it to one or two sentences that state who depends on the group and which changes are acceptable.

## Assign Definitions

The `group` attribute assigns a definition to a declared group:

```sidex
#[group(name = "mobile")]
#[request(name = "orders.Create", response = CreateOrderOutput)]
record CreateOrder {
    items: [OrderItem],
}

record CreateOrderOutput {
    order_id: string,
}

record OrderItem {
    product_id: string,
    quantity: u32,
}
```

Repeat the attribute to assign a definition to several groups. Naming a group that the manifest does not declare is an error, which every command that loads the bundle reports, including `sidex check`.

## Understand Membership

A promise about `CreateOrder` must also cover the types it uses. Changing `OrderItem` changes the JSON that mobile apps send, so `OrderItem` is part of what the `mobile` group promises, even without an attribute of its own.

Sidex therefore extends membership from each assigned definition to every definition of the same bundle that it uses, directly or indirectly, through:

- Field types and variant case payloads.
- Wrapped and aliased types.
- Type arguments, such as `OrderItem` in `[OrderItem]`. A [generic definition](/sidex/docs/language/types/#generics) of the bundle, such as `Page` in `Page<OrderItem>`, joins as well.
- The response type of a [request](/sidex/docs/guides/requests/).

In the example, `OrderItem` joins `mobile` as the type argument of `[OrderItem]`, and `CreateOrderOutput` joins as the response of `CreateOrder`. Without the `request` attribute, `CreateOrderOutput` would not belong to the group.

Membership flows only from a member to the definitions it uses. A web-only `ListOrders` request that returns `OrderItem` values does not join `mobile`. Assign groups to the entry points of an API, such as its requests, and let membership cover the types they use.

A definition used by members of several groups belongs to all of them. Changes to it must keep every one of their promises. For example, a change to `OrderItem` made for the web app must still keep the `mobile` promise.

Membership stops at bundle boundaries because each bundle declares its own groups. A type imported from a dependency does not join the importing bundle's groups. To document it, declare a group in the dependency and assign the type there.

Groups apply to whole definitions. Fields and variant cases are covered by the groups of their definition.

## Read Membership

The generated documentation of a member contains one paragraph per group, in name order, before the definition's own documentation. For `OrderItem`, the Rust generator produces:

```rust
/// **Group `mobile`:** Used by released mobile apps. Keep changes backward compatible for at least 12 months.
pub struct OrderItem {
    // ...
}
```

The other generators use the same wording in TypeScript JSDoc, Python docstrings and comments, and JSON Schema descriptions. If the definition has lifecycle attributes, the lifecycle notice comes before the group paragraphs, except in TypeScript, which renders lifecycle information as JSDoc tags after the text.

In Rust, the compiler can also check membership. The Rust generator can define a marker trait for a group, such as `MobileGroupMarker`, that the group's records, variants, and wrappers implement. Code can then use it as a bound to accept only types of the group. See [group marker traits](/sidex/docs/generation/rust/#group-marker-traits).

Custom tools can read membership from Sidex's [intermediate representation (IR)](/sidex/docs/guides/extending-sidex/#start-with-exported-ir). The `groups` field of a bundle maps each declared group name to an object with its `description`. The `groups` field of a definition is a sorted list of the names of the groups it belongs to, including membership through use.
