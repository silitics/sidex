---
sidebar_position: 6
---

# RPC API (Idea)

:::caution

This is nothing more than an idea at this point.

:::

## Streams

```sidex title="rpc.sidex"
opaque Stream<T>

wrapper StreamId: idx

record StreamCommand {
    id: StreamId,
    kind: StreamCommandKind,
}

variant StreamCommandKind {
    /// Closes the stream.
    Close,
    /// Sends a value.
    Send: SendCommand,
}

```

```sidex
wrapper RequestId: idx

record Request<T> {
    id: RequestId,
    data: T,
}

record Response<T> {
    id: RequestId,
    result: T,
}
```
