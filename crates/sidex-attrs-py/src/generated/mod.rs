/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod attrs {
    #![doc = "Typed attributes for the Python codegen target.\n\nPlugins use these schemas to drive code generation; the compiler parses\nsource `#[py(...)]` attributes against them and stores the result in the\nIR's `typed_attrs[\"py\"]` field. Downstream codegens deserialize that\nvalue directly into the generated Rust types via `serde_json::from_value`.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Python-specific attributes that may be applied to an `opaque` definition.\n\nSource form: `#[py(typ = \"module.Type\")]`. The `typ` value is the\nfully-qualified Python type path that the codegen emits as a type alias.\n"]
    #[derive(Clone, Debug)]
    pub struct OpaqueTypeAttrs {
        #[doc = "The Python type path (e.g. `\"datetime.datetime\"`).\n"]
        pub typ: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for OpaqueTypeAttrs {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::py::attrs::OpaqueTypeAttrs" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("typ" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("The Python type path (e.g. `\"datetime.datetime\"`).\n") . optional ()]))) . with_docs ("Python-specific attributes that may be applied to an `opaque` definition.\n\nSource form: `#[py(typ = \"module.Type\")]`. The `typ` value is the\nfully-qualified Python type path that the codegen emits as a type alias.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("py") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("opaque") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for OpaqueTypeAttrs {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for OpaqueTypeAttrs {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.typ
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for OpaqueTypeAttrs {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self { typ: __field_0 })
        }
    }
}
