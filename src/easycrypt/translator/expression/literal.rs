//!
//! Transpilation of YUL literals.
//!
use anyhow::Result;

use crate::easycrypt::syntax::expression::literal::integer::IntegerLiteral;
use crate::easycrypt::syntax::expression::literal::Literal as ECLiteral;
use era_yul::yul::lexer::token::lexeme::literal::Literal as YulLexerLiteral;
use era_yul::yul::parser::statement::expression::literal::Literal as YulParserLiteral;

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::translator::config::IConfig;
use crate::easycrypt::translator::Translator;

impl<C> Translator<C>
where
    C: IConfig,
{
    ///
    /// Transpile an arbitrary YUL literal into an EasyCrypt literal.
    ///
    pub fn translate_literal(lit: &YulParserLiteral) -> Result<Expression> {
        let one = ECLiteral::Int(IntegerLiteral {
            inner: String::from("1"),
        });
        let zero = ECLiteral::Int(IntegerLiteral {
            inner: String::from("0"),
        });

        let transpiled_literal = match &lit.inner {
            YulLexerLiteral::Boolean(b) => {
                if b == &era_yul::yul::lexer::token::lexeme::literal::boolean::Boolean::True {
                    one
                } else {
                    zero
                }
            }
            YulLexerLiteral::Integer(i) => match i {
                era_yul::yul::lexer::token::lexeme::literal::integer::Integer::Decimal {
                    inner,
                } => ECLiteral::Int(IntegerLiteral {
                    inner: inner.to_string(),
                }),
                era_yul::yul::lexer::token::lexeme::literal::integer::Integer::Hexadecimal {
                    inner,
                } => ECLiteral::Int(IntegerLiteral {
                    inner: crate::util::num::from_hex_literal(inner).to_string(),
                }),
            },

            era_yul::yul::lexer::token::lexeme::literal::Literal::String(s) => {
                ECLiteral::StringPlaceholder(s.to_string())
            }
        };

        C::wrap_literal(transpiled_literal)
    }
}
