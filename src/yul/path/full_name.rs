//!
//! Fully qualified name of a YUL variable or function, incorporating all
//! lexical scopes on the way from the root of YUL syntax tree.
//!

use era_yul::yul::parser::statement::expression::function_call::name::Name;

use crate::yul::path::Path;

///
/// Fully qualified name of a YUL variable or function, incorporating all
/// lexical scopes on the way from the root of YUL syntax tree.
///
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FullName {
    /// The name as given in the source code.
    pub name: Name,
    /// The path to the lexical scope containing this definition, incorporating
    /// all lexical scopes starting from the root of the YUL syntax tree.
    pub path: Path,
}

impl FullName {
    ///
    /// Create a new instance of [`FullName`].
    ///
    pub fn new(name: Name, path: Path) -> Self {
        Self { name, path }
    }

    ///
    /// Create a new instance of user-defined [`FullName`].
    ///
    pub fn custom(name: impl Into<String>, path: Path) -> Self {
        Self::new(Name::UserDefined(name.into()), path)
    }
}
