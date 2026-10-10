---
title: API Lifecycle
description: Document introductions, experimental contracts, and deprecations.
order: 2
---

A shared model evolves over time. Lifecycle attributes put migration information next to the affected definition, field, or variant case. Generators carry that information into target documentation, and `sidex check` diagnoses several inconsistent uses.

## Mark Introductions

```sidex
#[since(version = "1.0.0")]
record User {
    display_name: string,

    #[since(version = "1.2.0")]
    nickname?: string,
}
```

`since` describes when an item first appeared. Field metadata is independent of the enclosing definition's metadata. Use semantic versions when you want version-based checks.

## Mark Experimental Items

```sidex
#[unstable(feature = "search", note = "Ranking options may change.")]
record SearchOptions {
    query: string,
}
```

`feature` names the experimental surface, `note` explains its status, and `issue` can link to a tracking issue. This is metadata; the standard generators do not automatically create a feature gate from it.

## Deprecate with Migration Guidance

```sidex
#[deprecated(
    since = "2.0.0",
    remove_in = "3.0.0",
    note = "Use UserProfile instead."
)]
record LegacyUser {
    name: string,
}

record UserProfile {
    display_name: string,
}
```

`note` should tell readers what to use. `remove_in` records the planned removal version. A bare `#[deprecated]` is also valid.

When the bundle's version reaches or exceeds `remove_in` and the item still exists, `sidex check` reports an error. Comparisons require parseable semantic versions.

## Check Lifecycle Consistency

```sh
sidex check
```

The checker reports:

- Stable definitions referring to unstable types.
- Nondeprecated definitions referring to deprecated types.
- A field introduced before its enclosing definition.
- Deprecated items retained past their `remove_in` deadline.

Generated Rust uses native deprecation annotations where applicable. TypeScript emits documentation tags such as `@deprecated`, `@experimental`, and `@since`. Python receives documentation preludes. JSON Schema receives definition descriptions and deprecation metadata where supported.

Lifecycle attributes describe your model. They do not compare two model revisions or prove that a change is compatible.

## Evolve a Shared Contract

Compatibility depends on both readers and writers. For example, adding an optional field lets a new reader accept old JSON without that field. An old reader must still tolerate the new writer's additional field.

Review changes to required fields, field and case names, payload types, variant tagging, and validation constraints. Adding a variant case can also break exhaustive application code or an older reader.

For a rename, prefer an overlap period: introduce the replacement, retain the old field with a deprecation note, migrate readers and writers, then remove it in a planned release. Test representative JSON with every participating application.
