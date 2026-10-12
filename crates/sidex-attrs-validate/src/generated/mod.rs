/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod attrs {
    #![doc = "Typed attributes for the validation extension.\n\nEach `#[validate({ … }, message = \"…\", code = \"…\")]` is one rule.\nThe plugin runs in *repeated* mode: every source attribute parses into\nan independent record appended to `typed_attrs[\"validate\"]: [Value]`.\n\nPlugin codegens — Rust (`sidex-gen-rs`) and TypeScript (`sidex-gen-ts`) —\nconsume that array and emit per-rule validators that build a\n`ValidationReport` of `(path, code, message)` errors.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "One validation rule attached to a field.\n\nSource form:\n\n```sidex\n#[validate({ 0 <= _ <= 100 })]\n#[validate({ 1 <= _.size }, message = \"Required.\")]\n#[validate({ matches(_, \"...\") }, message = \"...\", code = \"format:phone\")]\n```\n"]
    #[derive(Clone, Debug)]
    pub struct FieldRule {
        #[doc = "The rule expression captured verbatim. Plugins parse this body with\nthe validation expression grammar.\n"]
        pub expr: ::sidex_ir::TokensValue,
        #[doc = "Optional human-readable message. When unset, the runtime picks a\ndefault keyed on the resolved error code (e.g. `\"Value must be at\nleast 1 character.\"` for `min_length`).\n"]
        pub message: ::std::option::Option<::std::string::String>,
        #[doc = "Override the error code emitted by recognized shapes (e.g. surface\na `regex` rule as `\"format:phone\"`).\n"]
        pub code: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for FieldRule {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::validate::attrs::FieldRule" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("expr" , :: sidex :: shape :: shape_of :: < :: sidex_ir :: TokensValue > ,) . with_docs ("The rule expression captured verbatim. Plugins parse this body with\nthe validation expression grammar.\n") , :: sidex :: shape :: FieldShape :: new ("message" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Optional human-readable message. When unset, the runtime picks a\ndefault keyed on the resolved error code (e.g. `\"Value must be at\nleast 1 character.\"` for `min_length`).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("code" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Override the error code emitted by recognized shapes (e.g. surface\na `regex` rule as `\"format:phone\"`).\n") . optional ()]))) . with_docs ("One validation rule attached to a field.\n\nSource form:\n\n```sidex\n#[validate({ 0 <= _ <= 100 })]\n#[validate({ 1 <= _.size }, message = \"Required.\")]\n#[validate({ matches(_, \"...\") }, message = \"...\", code = \"format:phone\")]\n```\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("validate") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("field") } , :: sidex :: shape :: Attr :: Path ("repeated")] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for FieldRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for FieldRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.expr)),
                1usize => {
                    self.message
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.code
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for FieldRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex_ir::TokensValue>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex_ir::TokensValue as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                expr: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `expr`"))?,
                message: __field_1,
                code: __field_2,
            })
        }
    }
    #[doc = "One validation rule attached to a record. The expression sees the whole\nrecord as `_`, with field accessors (`_.<field>`) for cross-field rules.\n"]
    #[derive(Clone, Debug)]
    pub struct RecordRule {
        #[doc = ""]
        pub expr: ::sidex_ir::TokensValue,
        #[doc = ""]
        pub message: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub code: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RecordRule {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::validate::attrs::RecordRule" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("expr" , :: sidex :: shape :: shape_of :: < :: sidex_ir :: TokensValue > ,) , :: sidex :: shape :: FieldShape :: new ("message" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . optional () , :: sidex :: shape :: FieldShape :: new ("code" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . optional ()]))) . with_docs ("One validation rule attached to a record. The expression sees the whole\nrecord as `_`, with field accessors (`_.<field>`) for cross-field rules.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("validate") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("record") } , :: sidex :: shape :: Attr :: Path ("repeated")] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for RecordRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RecordRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.expr)),
                1usize => {
                    self.message
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.code
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RecordRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex_ir::TokensValue>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex_ir::TokensValue as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                expr: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `expr`"))?,
                message: __field_1,
                code: __field_2,
            })
        }
    }
    #[doc = "One validation rule attached to a wrapper. The expression sees the\nwrapped value as `_`. Codegens may emit `try_new` constructors that\nrun validation up front when the wrapper has rules.\n"]
    #[derive(Clone, Debug)]
    pub struct WrapperRule {
        #[doc = ""]
        pub expr: ::sidex_ir::TokensValue,
        #[doc = ""]
        pub message: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub code: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for WrapperRule {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::validate::attrs::WrapperRule" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("expr" , :: sidex :: shape :: shape_of :: < :: sidex_ir :: TokensValue > ,) , :: sidex :: shape :: FieldShape :: new ("message" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . optional () , :: sidex :: shape :: FieldShape :: new ("code" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . optional ()]))) . with_docs ("One validation rule attached to a wrapper. The expression sees the\nwrapped value as `_`. Codegens may emit `try_new` constructors that\nrun validation up front when the wrapper has rules.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("validate") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("wrapper") } , :: sidex :: shape :: Attr :: Path ("repeated")] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for WrapperRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for WrapperRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.expr)),
                1usize => {
                    self.message
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.code
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for WrapperRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex_ir::TokensValue>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex_ir::TokensValue as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                expr: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `expr`"))?,
                message: __field_1,
                code: __field_2,
            })
        }
    }
    #[doc = "One validation rule attached to an opaque type. The expression sees the\nopaque value as `_`. Useful for primitive opaques whose representation\nis a string or number with target-specific semantics.\n"]
    #[derive(Clone, Debug)]
    pub struct OpaqueRule {
        #[doc = ""]
        pub expr: ::sidex_ir::TokensValue,
        #[doc = ""]
        pub message: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub code: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for OpaqueRule {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::validate::attrs::OpaqueRule" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("expr" , :: sidex :: shape :: shape_of :: < :: sidex_ir :: TokensValue > ,) , :: sidex :: shape :: FieldShape :: new ("message" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . optional () , :: sidex :: shape :: FieldShape :: new ("code" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . optional ()]))) . with_docs ("One validation rule attached to an opaque type. The expression sees the\nopaque value as `_`. Useful for primitive opaques whose representation\nis a string or number with target-specific semantics.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("validate") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("opaque") } , :: sidex :: shape :: Attr :: Path ("repeated")] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for OpaqueRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for OpaqueRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.expr)),
                1usize => {
                    self.message
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.code
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for OpaqueRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex_ir::TokensValue>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex_ir::TokensValue as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                expr: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `expr`"))?,
                message: __field_1,
                code: __field_2,
            })
        }
    }
}
