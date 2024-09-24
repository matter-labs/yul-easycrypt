//!
//! EasyCrypt AST node containing a definition of a new procedure.
//!

use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::signature::Signature;
use crate::easycrypt::syntax::statement::block::Block;

use super::Name;

///
/// EasyCrypt AST node containing a definition of a new procedure.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proc {
    /// Name of the procedure.
    pub name: Name,
    /// Signature of the procedure.
    pub signature: Signature,
    /// Definitions of the local variables.
    pub locals: Vec<Definition>,
    /// Body of the procedure.
    pub body: Block,
}
