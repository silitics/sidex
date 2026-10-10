---
title: Request Metadata
description: Pair an API request with its name and response type.
order: 3
---

A request contract has an input, a response, and a stable name used by the applications exchanging it. Sidex's `request` attribute records that relationship in typed metadata for tools that generate clients or inspect API models.

## Define a Request

```sidex
#[request(name = "tasks.CreateTask", response = CreateTaskOutput)]
record CreateTask {
    title: string,
    description?: string,
}

record CreateTaskOutput {
    task_id: string,
}
```

`name` is the wire-level request identifier. `response` is an unquoted type reference resolved against the bundle's definitions and imports.

Both arguments are required. The response must be a reachable record, variant, wrapper, or opaque definition.

## Read the Metadata

Generate the intermediate representation, or _IR_, which is Sidex's resolved model:

```sh
sidex generate ir generated/ir
```

The request definition's JSON entry contains `typedAttrs.request`, with its name and resolved response reference. A custom generator can read that entry to build a request registry or associate a method with a response type.

## Define the Transport Separately

The attribute describes a relationship between types. The standard CLI does not provide a request client, server, HTTP router, or OpenAPI generator. Your transport integration must define framing, authentication, errors, and delivery behavior.

Streaming requests are outside the current attribute's scope. Model transport-specific data explicitly or extend the tooling when you need a streaming contract.
