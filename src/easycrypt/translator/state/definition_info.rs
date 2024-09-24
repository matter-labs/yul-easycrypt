use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::unary::UnaryOpType;
use crate::easycrypt::syntax::r#type::Type;
use super::definition_location::DefinitionLocation;

/// TODO
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KindSpecific {
    Function,
    Proc,
    Variable,
}

/// TODO
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Builtin {
    BinaryOperation(BinaryOpType),
    UnaryOperation(UnaryOpType),
}

/// TODO
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Custom {
    pub specific: KindSpecific,
    pub location: DefinitionLocation,
}

/// TODO
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Description {
    Builtin(Builtin),
    Custom(Custom),
}

///
/// Static description of a definition (standard or non-standard) in transpiled
/// EasyCrypt code. Common part for all definitions.
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefinitionInfo {
    pub description: Description,
    pub r#type: Type,
}

impl DefinitionInfo {
    pub fn variable(location: &DefinitionLocation, typ: &Type) -> Self {
        Self {
            description: Description::Custom(Custom {
                specific: KindSpecific::Variable,
                location: location.clone(),
            }),
            r#type: typ.clone(),
        }
    }
}
