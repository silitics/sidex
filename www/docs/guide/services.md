---
sidebar_position: 4
---

# Services

**🚧 TODO**: This section is still under construction.

**🚧 TODO**: We need an analog to format mappings for (RPC) protocols or other means to communicate with an API.

A _service definition_ defines an API consisting of _methods_ which can be invoked.

An example of a service:

```sidex
service Api {
    fun hello(name: string) -> string
}
```

Arguments can be made optional with a `?` behind the name of the argument.
