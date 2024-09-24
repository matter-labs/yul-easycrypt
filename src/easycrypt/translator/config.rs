//!
//! Configuration for the translator.
//!

use anyhow::Result;
use era_yul::yul::parser::dialect::{DefaultDialect, Dialect};
use era_yul::yul::parser::statement::expression::function_call::name::Name;

use crate::easycrypt::path::step::Step;
use crate::easycrypt::path::Path as ECPath;
use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::call::FunctionCall;
use crate::easycrypt::syntax::expression::literal::Literal;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::reference::Reference;
use crate::easycrypt::translator::state::definition_info::DefinitionInfo;
use crate::yul::path::full_name::FullName;
use crate::yul::path::Path as YulPath;

pub trait IConfig: Default {
    type Dialect: Dialect;

    fn wrap_literal(literal: Literal) -> Result<Expression>;
    fn int_to_bool(expression: Expression) -> Result<Expression>;

    fn standard_definitions() -> Vec<(FullName, DefinitionInfo)>;
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ZksyncVerifierConfig {}

impl IConfig for ZksyncVerifierConfig {
    type Dialect = DefaultDialect;

    fn wrap_literal(literal: Literal) -> Result<Expression> {
        let wrapper_call = FunctionCall {
            target: Reference {
                identifier: String::from("of_int"),
                path: ECPath {
                    stack: vec![Step::Module(String::from("W256"))],
                },
            },
            arguments: vec![Expression::Literal(literal)],
        };
        Ok(Expression::ECall(wrapper_call))
    }

    fn int_to_bool(expression: Expression) -> Result<Expression> {
        Ok(Expression::ECall(FunctionCall {
            target: Reference {
                identifier: String::from("bool_of_uint256"),
                path: ECPath::empty(),
            },
            arguments: vec![expression],
        }))
    }

    fn standard_definitions() -> Vec<(FullName, DefinitionInfo)> {
        vec![(
            FullName {
                name: Name::Add,
                path: YulPath::empty(),
            },
            DefinitionInfo {
                description:
                    crate::easycrypt::translator::state::definition_info::Description::Builtin(
                        crate::easycrypt::translator::state::definition_info::Builtin::BinaryOperation(BinaryOpType::Add)),
                r#type: arrow_type(2),
            },
        )]
    }
}

fn arrow_type(arity: usize) -> Type {
    let inputs = Type::of_types(
        std::iter::repeat(def_type())
            .take(arity)
            .collect::<Vec<_>>()
            .into_iter(),
    );
    Type::Arrow(Box::from(inputs), Box::from(def_type()))
}
// fn arrow_type_proc(in_arity: usize, out_arity: usize) -> Type {
//     let inputs = Type::of_types(
//         &std::iter::repeat(def_type())
//             .take(in_arity)
//             .collect::<Vec<_>>(),
//     );
//     let outputs = Type::type_of_vec(
//         &std::iter::repeat(def_type())
//             .take(out_arity)
//             .collect::<Vec<_>>(),
//     );
//     Type::Arrow(Box::from(inputs), Box::from(outputs))
// }

fn def_type() -> Type {
    Type::DEFAULT.clone()
}
