//!
//! Types of binary operations in EasyCrypt AST.
//!

///
/// Types of binary operations in EasyCrypt AST.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOpType {
    /// `x + y`.
    Add,
    /// `x - y`.
    Sub,
    /// `x * y`.
    Mul,
    /// `x / y` or `0` if `y == 0`.
    Div,
    /// `x % y` or `0` if `y == 0`.
    Mod,

    /// `1` if `x == y`, `0` otherwise.
    Eq,

    /// bitwise "or" of `x` and `y`.
    Or,
    /// bitwise "xor" of `x` and `y`.
    Xor,
    /// bitwise "and" of `x` and `y`.
    And,
    /// `x` to the power of `y`
    Exp,
}
