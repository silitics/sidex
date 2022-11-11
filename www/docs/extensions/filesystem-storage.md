---
sidebar_position: 3
---

# Filesystem Storage (Idea)

The _filesystem storage_ extension can be used to generate code for storing entities in a hierarchical filesystem-like storage.

:::caution

This is nothing more than an idea at this point.

:::

```sidex
// `documents/<document-id>/latest/document.json`
// `documents/<document-id>/latest/children/*`
// `documents/<document-id>/versions/<version-number>/document.json`
// `documents/<document-id>/versions/<version-number>/children/*`
// `documents/_index.sqlite`
#[filesystem(path = "documents", versioned)]
record Document {
    #[filesystem(id)]
    id: DocumentId,
    #[index]
    created: DateTime,
    #[filesystem(children(type = Section))]
    sections: [SectionId],
}

#[filesystem(path = "sections")]
record Section {
    #[filesystem(id)]
    id: SectionId,
    name: string,
}
```

```sidex
#[filesystem(path = "images")]
record Image {
    #[filesystem(id)]
    filename: string,
    #[filesystem(raw)]
    data: bytes,
}
```
