---
sidebar_position: 5
---

# HTTP APIs (Draft)

:::caution

This is a work-in-progress draft.

:::

These days, HTTP APIs are an integral part of many applications – even beyond the web.
What makes HTTP APIs an excellent choice is the outstanding ecosystem of tools and libraries surrounding them.
Catalyzed by the standardization efforts of the [OpenAPI Initiative](https://www.openapis.org), a [plethora of tools](https://openapi.tools/) has emerged providing of-the-shelf solutions for generating clients, servers, and documentation, for testing, and for so much more.
Sidex taps into this broad ecosystem by providing a generator for OpenAPI schemas based on Sidex definitions.
Sidex HTTP APIs are defined by mapping methods of [interfaces](../../docs/guide/interfaces.md) to HTTP paths and [request methods](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods).

#### Example: User Management

Let's start with a simple example of an API for managing users, so you get a rough idea how a Sidex HTTP API definition may look like.
The API we are going to define should provide a way to list, create, and delete users, as well as to retrieve information about individual users and profile pictures.
The core of a Sidex definition for such an API may look as follows:

```sidex
#[http("/users")]
interface Users {
    #[http(get)]
    fun list(skip?: u32, limit?: u32) -> [User];

    #[http(post("create"))]
    fun create(user: CreateUserForm) -> UserId;
}

#[http("/users/{user_id}")]
interface User(user_id: UserId) {
    #[http(get)]
    fun get() -> User;

    #[http(delete)]
    fun delete();

    #[http(get("picture.jpg", content_type="image/jpeg"))]
    fun picture() -> bytes;
}
```

This API allows listing users via the `/users` endpoint and supports pagination.
For instance, requesting `/users?limit=20` will return a JSON array of the first 20 users.
To create a user, a `POST` request must be made to `/users/create` containing [form data](https://developer.mozilla.org/en-US/docs/Learn/Forms), e.g., via an HTML `<form>` element.
Here, the type <code className="type-name">CreateUserForm</code> specifies the fields of the form and is defined as follows:

```sidex
#[http(form)]
record CreateUserForm {
    name: string,
    password: string,
    #[http(size_limit="100KB")]
    picture: UploadedFile,
}
```

So, the newly created user needs to be given a `name`, a `password`, and a profile `picture`.

For each user, an endpoint `/users/{user_id}` is provided, where `{user_id}` is a unique id of the user.
For instance, if users are stored in a database using an integer as a primary key, `/users/5` does correspond to the user with the id `5`.
For each user, requesting `/users/{user_id}` returns information about the user as a <code className="type-name">User</code> JSON object.
In addition, sending a `DELETE` request to this path will delete the user, and requesting `/users/{user_id}/picture.jpg` will return the profile picture.

Now, based on this definition, Sidex is able to generate an OpenAPI schema which can then be used with all the [OpenAPI-based tools](https://openapi.tools/).
Among other things, this allows [generating clients and servers for a variety of different programming languages](https://openapi-generator.tech/).
The advantage of using Sidex over writing an OpenAPI schema manually lies in the Sidex language itself and the additional type information present in the Sidex definition.
We believe that the additional type information will eventually enable Sidex to generate better clients and servers because interfaces and types are closer to the constructs available in most programming languages.

## Parsing Requests

_Extrators_ are used to extract information from HTTP requests.
For instance, the type <code className="type-name">CreateUserForm</code> defined above extracts form data from the body of an HTTP request.
Extractors can also extract information from query parameters, headers, and cookies.
For example, instead of the arguments `skip` and `limit` on the `list` method, we can also define a type:

```sidex
#[http(extractor)]
record Pagination {
    skip?: u32,
    limit?: u32,
}
```

This type can then be used as an argument of a method (or of an interface) and extracts the query parameters for pagination.
This means that we can reuse the type for multiple endpoints and extend it without touching every endpoint individually.

By default, numbers, booleans, and strings are interpreted as query parameters.

To extract headers and cookies, the `header` and `cookie` attribute must be used:

```sidex
#[http(extractor)]
record RequestInfo {
    #[http(header)]
    user_agent: string,
    #[http(cookie)]
    session_id?: string,
}
```

The names of query parameters, headers, and cookies are automatically obtained from the field names, so `user_agent` becomes `User-Agent`.
To use a different name, provide an argument to the attributes.
For example:

```sidex
#[http(extractor)]
record SessionInfo {
    #[http(header("X-Session-Id"))]
    id: string,
}
```

To mark something explicitly as a query parameter, you can use the `query` attribute which also accepts a name.

By default, <code className="builtin-type">bytes</code> will be assumed to be Base64 encoded when extract from a header, cookie, or query parameter.

**🚧 TODO:** How do we handle other types, e.g., record and variant types?

Extractors can also be defined using wrapper types.
For example:

```sidex
#[http(header("User-Agent"))]
wrapper UserAgent: string
```

### Parsing Bodies

While query parameters, headers, and cookies are straightforward to parse, things get a bit more complicated with the request body.
The body of a request may be too large to fit into memory and contain data encoded in various formats.

#### Streaming

To support streaming the body, there are three auxiliary types:

```sidex title="request.sidex"
/// Body is stored in a temporary file.
#[json(type="string")]
opaque UploadedFile

/// Body is provided as a bytes stream to the application.
///
/// The default content type is `application/octet-stream`.
opaque BytesStream

/// Body is provided as a text stream to the application.
///
/// The default content type is `text/plain`.
opaque TextStream

/// A stream of type `T`.
opaque Stream<T>
```

If streaming is not used, the size of request bodies should be limited by the server framework.

#### Forms

The bodies of form submissions contain key-value pairs and may have two encodings:

1. `application/x-www-form-urlencoded`: Usually, this encoding is used if no files are uploaded.
2. `multipart/form-data`: This encoding must be used if files are uploaded.

To extract form bodies, special `form` extractor types must be defined.
We already saw <code className="type-name">CreateUserForm</code> as an example of such a type.
By default and if possible, both encodings as well as `application/json` are supported.
For JSON, the fields are stored in a JSON object encoding files with Base64.
This enables a more flexible usage of the API.

The encodings can be constrained using attributes.
Here are some examples:

- `#[http(form(multipart))]`
- `#[http(form(urlencoded, multipart))]`
- `#[http(form(multipart, json))]`

Note that defining `form` record types involves processing the whole request before invoking the handler because otherwise the type could not be constructed.
Multipart content can also be streamed.
To this end, define a multipart variant type, e.g.:

```sidex
#[http(multipart)]
variant Part {
    Username: string,
    Password: string,
    File: BytesStream,
}
```

This type can then be used with <code className="type-name">Stream</code> as defined above using <code><span className="type-name">Stream</span>&lt;<span className="type-name">Part</span>&gt;</code>.

To limit the size of certain fields use `#[http(max_size="...")]`.

### WebSockets

[WebSockets](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API) are not supported by OpenAPI, nevertheless, they can be used in Sidex.

```sidex title="request.sidex"
/// Request to establish a WebSocket connection.
opaque WebSocket
```

**🚧 TODO:** What do we do when generating the OpenAPI schema?

## Constructing Responses

In principle, the encoding of the return type, in this case <code>[<span className="type-name">User</span>]</code>, can be controlled by the [`Accept`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept) header of the request.

By default, numbers, booleans, and strings are returned as `text/plain`.

Bytes are `application/octet-stream`.

Complex objects are `application/json`.

### Status Codes

Return a response object.

```sidex
#[http(response)]
variant Response<T> {
    #[http(status=200)]
    Ok: T,
    #[http(status=401)]
    Unauthorized,
    #[http(status=403)]
    Forbidden,
    #[http(status=404)]
    NotFound,
    #[http(status=500)]
    InternalError,
}
```

```sidex
#[http(response)]
record Response {
    #[http(header("X-Session-Id"))]
    session_id?: string,
    #[http(status)]
    status: u16,
    body: BytesStream,
}
```

`Result` and `Option` are also supported.

- `Option`: If `None` is returned, then a `404` is generated.
- `Result`: Simply destructs the result and responds with the value or the error.

#### Global Error Types

```sidex
#[http(error)]
variant Error {
    #[http(status=401)]
    Unauthorized,
    #[http(status=403)]
    Forbidden,
    #[http(status=404)]
    NotFound,
    #[http(status=500)]
    InternalError,
}
```
