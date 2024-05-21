#![feature(assert_matches)]

pub mod to_wit;
pub mod translations;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translations::ConversionOptions;
    use pretty_assertions::assert_eq;
    use std::{fs, path::Path};

    fn compare(path: &str) {
        let webidl_input =
            fs::read_to_string(Path::new(&format!("./tests-input/{path}.idl"))).unwrap();
        let wit_input =
            fs::read_to_string(Path::new(&format!("./tests-input/{path}.wit"))).unwrap();

        let webidl_ast = weedle::parse(&webidl_input).unwrap();
        let wit_ast =
            translations::webidl_to_wit(webidl_ast, ConversionOptions::default()).unwrap();
        let wit_output = wit_ast.to_string();

        assert_eq!(wit_input, wit_output)
    }

    #[test]
    fn enum_() {
        compare("enum");
    }

    #[test]
    fn resource() {
        compare("resource");
    }

    #[test]
    fn record() {
        compare("record");
    }

    #[test]
    fn type_() {
        compare("type");
    }

    #[test]
    fn webgpu() {
        compare("webgpu");
    }
}
