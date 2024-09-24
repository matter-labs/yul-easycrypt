//!
//! EasyCrypt AST node containing a definition of a new variable.
//!

use crate::easycrypt::path::Path;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::Name;

use super::reference::Reference;

///
/// EasyCrypt AST node containing a definition of a new variable.
/// See also: [`DefinitionInfo`] and [`DefinitionPoint`].
///
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Definition {
    /// Name of the variable.
    pub identifier: Name,

    /// Type of the variable, if the definition is annotated with one.
    pub r#type: Option<Type>,
}

impl Definition {
    pub fn new(identifier: Name, r#type: Option<Type>) -> Self {
        Self { identifier, r#type }
    }

    ///
    /// Produce a reference node with the same name.
    ///
    pub fn reference(&self) -> Reference {
        Reference {
            identifier: self.identifier.clone(),
            path: Path::empty(),
        }
    }

    ///
    /// Explicitly provided type of this definition, or a default type.
    ///
    pub fn get_effective_type(&self) -> Type {
        if let Some(typ) = &self.r#type {
            typ
        } else {
            Type::DEFAULT
        }
        .clone()
    }
}
