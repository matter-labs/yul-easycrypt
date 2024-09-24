//!
//! EasyCrypt AST node containing a reference to a previously defined variable.
//!

use crate::easycrypt::path::Path;
use crate::easycrypt::syntax::Name;

///
/// EasyCrypt AST node containing a reference to a previously defined variable.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reference {
    /// Name of the variable.
    pub identifier: Name,
    /// Destination path.
    pub path: Path,
}
