---
sidebar_position: 5
---

# REST API (Idea)

This extension standardizes attributes for HTTP REST (_Representational State Transfer_) APIs by mapping HTTP endpoints to functions of a service. This mapping is achieved by REST-specific attributes such as `#[get("/users/<id>/info")]`.

:::caution

This is nothing more than an idea at this point.

:::

🚧 The idea is to standardize attributes similar to those provided by [Rocket](https://rocket.rs/).

## Why standardize REST attributes?

REST APIs are at the core of many applications. By standardizing attributes for REST APIs, we enable interoperability between different programming languages and platforms (assuming the existence of appropriate backends).
