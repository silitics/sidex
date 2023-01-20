---
sidebar_position: 4
---

# Interfaces

An _interface definition_ defines an API consisting of _methods_ which can be invoked.

```sidex
interface Api {
    fun say_hello(name: string) -> string
}
```

Arguments can be made optional with a `?` behind the name of the argument.
