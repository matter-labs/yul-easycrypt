use era_yul::yul::parser::dialect::Dialect;
use era_yul::yul::parser::statement::function_definition::FunctionDefinition;

use super::path::full_name::FullName;

// FIXME: rewrite so that we operate on easycrypt?
//pub mod calling_dependencies;
pub mod collect_definitions;
//pub mod common;
//pub mod functions;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Unit,
    Atomic(era_yul::yul::parser::r#type::Type),
    Function {
        arguments_type: Vec<Type>,
        return_type: Vec<Type>,
    },
}

impl Default for Type {
    fn default() -> Self {
        Type::Atomic(era_yul::yul::parser::r#type::Type::UInt(256))
    }
}

///
/// Basic description of a YUL function.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function<P>
where
    P: Dialect,
{
    pub full_name: FullName,
    pub body: FunctionDefinition<P>,
    pub typ: Type,
}

///
/// Basic description of YUL variable.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: FullName,
    pub typ: Type,
}

///
/// Basic description of a YUL definition.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Definition<P>
where
    P: Dialect,
{
    Function(Function<P>),
    Variable(Variable),
}
