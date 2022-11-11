---
sidebar_position: 3
---

# ORM Storage (Idea)

Extension for storing objects in a relational database using an _object relation mapping_ (ORM). This extension specifies attributes for mapping Sidex types to their representation in a relational database (e.g., [MySQL](https://www.mysql.com) or [PostgreSQL](https://www.postgresql.org)).

:::caution

This is nothing more than an idea at this point.

:::

🚧 The idea is to standardize attributes similar to those provided by [SeaORM](https://www.sea-ql.org/SeaORM) or [Diesel](https://diesel.rs).

## Why standardize ORM attributes?

Relational databases are at the core of many applications. By standardizing attributes for ORMs, we enable interoperability between different programming languages and platforms (assuming the existence of appropriate backends).
