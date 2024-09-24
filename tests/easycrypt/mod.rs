#![cfg(test)]

pub mod translator;

use era_yul::yul::lexer::Lexer;
use era_yul::yul::parser::dialect::DefaultDialect;
use era_yul::yul::parser::statement::object::Object;

use yul_easycrypt::easycrypt::syntax::module::Module;
use yul_easycrypt::easycrypt::translator::config::ZksyncVerifierConfig;
use yul_easycrypt::easycrypt::translator::Translator;

use crate::equals_modulo_formatting;

pub fn translate(input: &str) -> Module {
    let mut lexer = Lexer::new(input.to_string());
    let object = Object::<DefaultDialect>::parse(&mut lexer, None).unwrap();
    Translator::<ZksyncVerifierConfig>::new()
        .translate(&object)
        .unwrap()
}

pub fn test_translation(input: &str, expected: &str) {
    let module = translate(input);
    let formatted = format!("{:}", module);
    //println!("{}", formatted);
    assert!(
        equals_modulo_formatting(&formatted, expected),
        "--- Expected --- \n{}\n--- Got ---\n{}",
        expected,
        formatted
    )
}
