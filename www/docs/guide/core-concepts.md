---
sidebar_position: 1
---

# Core Concepts

As a quick reference and first overview, here a list of Sidex's _core concepts_ each with a short description:

<dl>
  <dt>
    <strong>Bundle</strong>
  </dt>
  <dd>
    A <em>bundle</em> is a flat collection of schemas evolving together. A Sidex bundle defines a data model, an API, or both. Every bundle has its own directory with a <em>bundle manifest</em> <code>sidex.toml</code> and a <code>schemas</code> directory for schemas.
  </dd>
  <dt>
    <strong>Schema</strong>
  </dt>
  <dd>
    A <em>schema</em> groups together multiple type and/or service definitions under a common namespace within a bundle. Schemas are used to organize such definitions into groups of related domain or functionality. Each schema is defined in a file <code>schema_name.sidex</code> within in the <code>schemas</code> directory of its bundle where <code>schema_name</code> is the name of the schema.
  </dd>
  <dt>
    <strong>Type Definition</strong>
  </dt>
  <dd>
    A <em>type definition</em> defines a <em>data type</em> constraining the form data may take. Sidex's type system supports a variety of different data types and even allows you to define your own primitives. For further details, read the <a href="data-types">section about data types</a>.
  </dd>
  <dt>
    <strong>Service Definition</strong>
  </dt>
  <dd>
    A <em>service definition</em> defines a <em>service</em> specifying <em>methods</em> which can be invoked with certain <em>arguments</em> and return a <em>result</em>. Service definitions are at the core of API definitions. For further details, read the <a href="services">section about services</a>.
  </dd>
  <dt>
    <strong>Language Mapping</strong>
  </dt>
  <dd>
    A <em>language mapping</em> defines how Sidex's data types and services are mapped to native types of a programming language. Language mappings are usually <em>format-independent</em>, i.e., independent of any interchange format.
  </dd>
  <dt>
    <strong>Format Mapping</strong>
  </dt>
  <dd>
    A <em>format mapping</em> defines how Sidex's data types are mapped to the primitives and structures of an interchange format. Format mappings are usually <em>language-independent</em>, i.e., independent of any programming language.
  </dd>
</dl>
