//!
//! EasyCrypt AST node containing a definition of a function.
//!

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::signature::Signature;

use super::Name;

///
/// EasyCrypt AST node containing a definition of a function.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    /// Name of the function.
    pub name: Name,
    /// Function signature.
    pub signature: Signature,
    /// Function body, which can only be a single expression.
    pub body: Expression,
}
