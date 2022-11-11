---
sidebar_position: 2
---

# Versioning (Draft)

Versioning and stability guarantees are central when working with data models and API definitions where backwards-compatibility is key to not breaking systems in production. Hence, this extension standardizes attributes for versioning.

## Deprecation

Data types, services, fields, variants, and methods can be _deprecated_:

```sidex
#[deprecated]
#[deprecated(since = "…")]
```

## Unstable

```sidex
#[unstable]
```
