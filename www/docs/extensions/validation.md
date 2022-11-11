---
sidebar_position: 3
---

# Validation (Idea)

:::caution

This is nothing more than an idea at this point.

:::

This extension defines standardized attributes for validating data.

```sidex
#[validate(_.min <= _.max)]
record Interval {
    #[validate(-5 <= _ <= 10)]
    min: i32,
    max: i32,
}

#[validate("/<SOME-REGEX>/")]
wrapper Email: string
```
