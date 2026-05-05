//! Typed attributes for API stability and lifecycle.
//!
//! Three plugins drive this crate:
//!
//! - `#[deprecated(...)]` — the item is deprecated.
//! - `#[unstable(...)]`   — the item's contract may still change.
//! - `#[since(...)]`      — the version in which the item was introduced.
//!
//! The schema for each plugin lives in
//! [`lib/api/schemas/attrs.sidex`](https://github.com/silitics/sidex/blob/main/crates/sidex-core/lib/api/schemas/attrs.sidex).
//! The compiler validates source attributes against that schema and stores the
//! parsed result on the IR node's `typed_attrs` map. This crate exposes
//! convenience extractors that pull values back out of `typed_attrs` and a
//! [`Stability`] rollup that combines all three for a single position.
//!
//! Codegens additionally call [`render_doc_prelude`] to prepend a stability
//! summary to the rendered doc comment. The prelude is intentionally
//! identical across targets so consumers see consistent wording.

mod check;
mod generated;

use std::collections::HashMap;

pub use check::check;
pub use generated::attrs as raw;
use serde_json::Value;
use sidex_ir as ir;

const PLUGIN_DEPRECATED: &str = "deprecated";
const PLUGIN_UNSTABLE: &str = "unstable";
const PLUGIN_SINCE: &str = "since";

/// Normalized deprecation metadata. The four target-specific records in the
/// schema (`DefDeprecated`, `FieldDeprecated`, `VariantDeprecated`,
/// `SchemaDeprecated`) all share this shape; we expose a single struct so
/// callers don't have to switch on position.
#[derive(Clone, Debug, Default)]
pub struct Deprecated {
    pub since: Option<String>,
    pub note: Option<String>,
    pub remove_in: Option<String>,
}

/// Normalized unstable metadata.
#[derive(Clone, Debug, Default)]
pub struct Unstable {
    pub feature: Option<String>,
    pub note: Option<String>,
    pub issue: Option<String>,
}

/// Normalized `#[since]` metadata.
#[derive(Clone, Debug)]
pub struct Since {
    pub version: String,
}

/// Convenience rollup: every stability attribute applicable to a single IR
/// node, extracted in one pass. Codegens typically want all three together
/// to render the doc prelude.
#[derive(Clone, Debug, Default)]
pub struct Stability {
    pub deprecated: Option<Deprecated>,
    pub unstable: Option<Unstable>,
    pub since: Option<Since>,
}

impl Stability {
    /// Whether this node carries any stability annotation.
    pub fn is_empty(&self) -> bool {
        self.deprecated.is_none() && self.unstable.is_none() && self.since.is_none()
    }
}

/// Pull the rollup off any IR node's `typed_attrs`. The schema records share
/// a common shape per plugin, so a single deserialization path covers
/// def / field / variant / schema positions.
pub fn stability_of(typed_attrs: &HashMap<String, Value>) -> Stability {
    Stability {
        deprecated: extract_deprecated(typed_attrs),
        unstable: extract_unstable(typed_attrs),
        since: extract_since(typed_attrs),
    }
}

/// Extract just the deprecation metadata, or `None` when the node isn't
/// deprecated.
pub fn deprecated_of(typed_attrs: &HashMap<String, Value>) -> Option<Deprecated> {
    extract_deprecated(typed_attrs)
}

/// Extract just the unstable metadata, or `None` when the node is stable.
pub fn unstable_of(typed_attrs: &HashMap<String, Value>) -> Option<Unstable> {
    extract_unstable(typed_attrs)
}

/// Extract just the `#[since]` metadata, or `None` when no version is
/// recorded for this node.
pub fn since_of(typed_attrs: &HashMap<String, Value>) -> Option<Since> {
    extract_since(typed_attrs)
}

/// Render a Markdown stability prelude suitable for prepending to the rendered
/// doc comment. Returns `""` when there's nothing to say.
///
/// The wording is fixed — uniform across codegens means uniform editor
/// surfacing across languages.
pub fn render_doc_prelude(stability: &Stability) -> String {
    let mut lines: Vec<String> = Vec::new();
    if let Some(dep) = &stability.deprecated {
        let mut head = String::from("**Deprecated");
        if let Some(since) = &dep.since {
            head.push_str(" since ");
            head.push_str(since);
        }
        if let Some(remove_in) = &dep.remove_in {
            head.push_str("; will be removed in ");
            head.push_str(remove_in);
        }
        head.push_str(".**");
        if let Some(note) = &dep.note {
            head.push(' ');
            head.push_str(note);
        }
        lines.push(head);
    }
    if let Some(u) = &stability.unstable {
        let mut head = String::from("**Unstable");
        if let Some(feature) = &u.feature {
            head.push_str(" (feature `");
            head.push_str(feature);
            head.push_str("`)");
        }
        head.push_str(".**");
        if let Some(note) = &u.note {
            head.push(' ');
            head.push_str(note);
        }
        if let Some(issue) = &u.issue {
            head.push_str(" Tracking: ");
            head.push_str(issue);
        }
        lines.push(head);
    }
    if let Some(since) = &stability.since {
        lines.push(format!("**Available since:** {}.", since.version));
    }
    if lines.is_empty() {
        return String::new();
    }
    let mut out = lines.join("\n");
    out.push_str("\n\n");
    out
}

// ---- Convenience overloads for IR positions --------------------------------

/// Read [`Stability`] off a `Def`.
pub fn stability_of_def(def: &ir::Def) -> Stability {
    stability_of(&def.typed_attrs)
}

/// Read [`Stability`] off a `Field`.
pub fn stability_of_field(field: &ir::Field) -> Stability {
    stability_of(&field.typed_attrs)
}

/// Read [`Stability`] off a `Variant` case.
pub fn stability_of_variant(variant: &ir::Variant) -> Stability {
    stability_of(&variant.typed_attrs)
}

/// Read [`Stability`] off a `Schema`.
pub fn stability_of_schema(schema: &ir::Schema) -> Stability {
    stability_of(&schema.typed_attrs)
}

// ---- internals -------------------------------------------------------------

fn extract_deprecated(typed_attrs: &HashMap<String, Value>) -> Option<Deprecated> {
    let value = typed_attrs.get(PLUGIN_DEPRECATED)?;
    // The schema record is identical across targets, so a single deserialize
    // call suffices regardless of position.
    serde_json::from_value::<DeprecatedWire>(value.clone())
        .ok()
        .map(|w| {
            Deprecated {
                since: w.since,
                note: w.note,
                remove_in: w.remove_in,
            }
        })
}

fn extract_unstable(typed_attrs: &HashMap<String, Value>) -> Option<Unstable> {
    let value = typed_attrs.get(PLUGIN_UNSTABLE)?;
    serde_json::from_value::<UnstableWire>(value.clone())
        .ok()
        .map(|w| {
            Unstable {
                feature: w.feature,
                note: w.note,
                issue: w.issue,
            }
        })
}

fn extract_since(typed_attrs: &HashMap<String, Value>) -> Option<Since> {
    let value = typed_attrs.get(PLUGIN_SINCE)?;
    serde_json::from_value::<SinceWire>(value.clone())
        .ok()
        .map(|w| Since { version: w.version })
}

#[derive(serde::Deserialize)]
struct DeprecatedWire {
    #[serde(default)]
    since: Option<String>,
    #[serde(default)]
    note: Option<String>,
    #[serde(default)]
    remove_in: Option<String>,
}

#[derive(serde::Deserialize)]
struct UnstableWire {
    #[serde(default)]
    feature: Option<String>,
    #[serde(default)]
    note: Option<String>,
    #[serde(default)]
    issue: Option<String>,
}

#[derive(serde::Deserialize)]
struct SinceWire {
    version: String,
}
