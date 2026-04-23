#[cfg(test)]
extern crate self as sidex_codegen;

mod code;

pub use code::{Code, Part, ToCode};
pub use sidex_codegen_macros::quote;

#[cfg(test)]
mod tests {
    use crate::{Code, quote};

    #[test]
    fn test_simple_literal() {
        let code = quote!("hello world");
        assert_eq!(code.to_string(), "hello world");
    }

    #[test]
    fn test_simple_interpolation() {
        let name = Code::from("Foo");
        let code = quote!("class @name:");
        assert_eq!(code.to_string(), "class Foo:");
    }

    #[test]
    fn test_multiline_dedent() {
        let code = quote!(
            "
            hello
            world
        "
        );
        assert_eq!(code.to_string(), "hello\nworld");
    }

    #[test]
    fn test_interpolation_with_indent() {
        let body = Code::from("return x + y");
        let code = quote!(
            "
            def foo():
                @body
        "
        );
        assert_eq!(code.to_string(), "def foo():\n    return x + y");
    }

    #[test]
    fn test_multiline_interpolation_column_aligned() {
        let args = Code::from("x: int,\ny: int,\nz: int");
        let code = quote!("def foo(@args):");
        assert_eq!(
            code.to_string(),
            "def foo(x: int,\n        y: int,\n        z: int):"
        );
    }

    #[test]
    fn test_nested_quote() {
        let inner = quote!(
            "
            x = 1
            y = 2
            return x + y
        "
        );
        let code = quote!(
            "
            def foo():
                @inner
        "
        );
        assert_eq!(
            code.to_string(),
            "def foo():\n    x = 1\n    y = 2\n    return x + y"
        );
    }

    #[test]
    fn test_vertical_iteration() {
        let items = vec![
            Code::from("x = 1"),
            Code::from("y = 2"),
            Code::from("return x + y"),
        ];
        let code = quote!(
            "
            def foo():
                @(@items)*
        "
        );
        assert_eq!(
            code.to_string(),
            "def foo():\n    x = 1\n    y = 2\n    return x + y"
        );
    }

    #[test]
    fn test_vertical_iteration_with_separator() {
        let items = vec![
            Code::from("\"apple\""),
            Code::from("\"banana\""),
            Code::from("\"cherry\""),
        ];
        let code = quote!(
            "
            fruits = [
                @(@items),*
            ]
        "
        );
        assert_eq!(
            code.to_string(),
            "fruits = [\n    \"apple\",\n    \"banana\",\n    \"cherry\"\n]"
        );
    }

    #[test]
    fn test_horizontal_iteration() {
        let args = vec![
            Code::from("x: int"),
            Code::from("y: int"),
            Code::from("z: int"),
        ];
        let code = quote!("def foo(@(@args), +):");
        assert_eq!(code.to_string(), "def foo(x: int, y: int, z: int):");
    }

    #[test]
    fn test_escaped_at() {
        let code = quote!("email@@example.com");
        assert_eq!(code.to_string(), "email@example.com");
    }

    #[test]
    fn test_empty_iteration() {
        let items: Vec<Code> = vec![];
        let code = quote!("@(@items)*");
        assert_eq!(code.to_string(), "");
    }

    #[test]
    fn test_single_item_iteration() {
        let items = vec![Code::from("only_one")];
        let code = quote!("@(@items),*");
        assert_eq!(code.to_string(), "only_one");
    }

    #[test]
    fn test_iteration_with_body_pattern() {
        let names = vec![Code::from("x"), Code::from("y"), Code::from("z")];
        let types = vec![Code::from("int"), Code::from("str"), Code::from("float")];
        let code = quote!(
            "
            @(@names: @types)*
        "
        );
        assert_eq!(code.to_string(), "x: int\ny: str\nz: float");
    }

    #[test]
    fn test_vertical_multiline_items() {
        let methods = vec![
            quote!(
                "
                def a(self):
                    pass
            "
            ),
            quote!(
                "
                def b(self):
                    pass
            "
            ),
        ];
        let code = quote!(
            "
            class Foo:
                @(@methods)*
        "
        );
        assert_eq!(
            code.to_string(),
            "class Foo:\n    def a(self):\n        pass\n    def b(self):\n        pass"
        );
    }

    #[test]
    fn test_blank_line_in_template() {
        let code = quote!(
            "
            hello

            world
        "
        );
        assert_eq!(code.to_string(), "hello\n\nworld");
    }

    #[test]
    fn test_complex_python_class() {
        let class_name = Code::from("Point");
        let base = Code::from("pydantic.BaseModel");
        let fields = vec![Code::from("x: float"), Code::from("y: float")];
        let code = quote!(
            "
            class @class_name(@base):
                @(@fields)*
        "
        );
        assert_eq!(
            code.to_string(),
            "class Point(pydantic.BaseModel):\n    x: float\n    y: float"
        );
    }
}
