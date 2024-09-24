//!
//! Visitor pattern for EasyCrypt syntax tree.
//!

use super::syntax::definition::Definition;
use super::syntax::expression::binary::BinaryOpType;
use super::syntax::expression::call::FunctionCall;
use super::syntax::expression::literal::integer::IntegerLiteral;
use super::syntax::expression::literal::Literal;
use super::syntax::expression::unary::UnaryOpType;
use super::syntax::expression::Expression;
use super::syntax::function::Function;
use super::syntax::module::definition::TopDefinition;
use super::syntax::module::Module;
use super::syntax::proc::Proc;
use super::syntax::r#type::Type;
use super::syntax::reference::Reference;
use super::syntax::signature::Signature;
use super::syntax::statement::block::Block;
use super::syntax::statement::call::ProcCall;
use super::syntax::statement::if_conditional::IfConditional;
use super::syntax::statement::while_loop::WhileLoop;
use super::syntax::statement::Statement;
use anyhow::Result;

///
/// Describes the visitor pattern for EasyCrypt syntax tree.
///
pub trait Visitor {
    ///
    /// Visit binary operation node in an EasyCrypt syntax tree.
    ///
    fn visit_binary_op_type(&mut self, op: &BinaryOpType) -> Result<()>;

    ///
    /// Visit block of statements in an EasyCrypt syntax tree.
    ///
    fn visit_block(&mut self, block: &Block) -> Result<()>;

    ///
    /// Visit a variable definition in an EasyCrypt syntax tree.
    ///
    fn visit_definition(&mut self, definition: &Definition) -> Result<()>;

    ///
    /// Visit an arbitrary expression in an EasyCrypt syntax tree.
    ///
    fn visit_expression(&mut self, expression: &Expression) -> Result<()>;

    ///
    /// Visit a function definition in an EasyCrypt syntax tree.
    ///
    fn visit_function(&mut self, function: &Function) -> Result<()>;

    ///
    /// Visit a function call in an EasyCrypt syntax tree.
    ///
    fn visit_function_call(&mut self, call: &FunctionCall) -> Result<()>;

    ///
    /// Visit an integer literal in an EasyCrypt syntax tree.
    ///
    fn visit_integer_literal(&mut self, int_literal: &IntegerLiteral) -> Result<()>;

    ///
    /// Visit a conditional statement in an EasyCrypt syntax tree.
    ///
    fn visit_if_conditional(&mut self, conditional: &IfConditional) -> Result<()>;

    ///
    /// Visit an arbitrary literal in an EasyCrypt syntax tree.
    ///
    fn visit_literal(&mut self, literal: &Literal) -> Result<()>;

    ///
    /// Visit a module definition in an EasyCrypt syntax tree.
    ///
    fn visit_module(&mut self, module: &Module) -> Result<()>;

    ///
    /// Visit a top-level definition in a module in an EasyCrypt syntax tree.
    ///
    fn visit_module_definition(&mut self, module: &TopDefinition) -> Result<()>;

    ///
    /// Visit a call to a procedure in an EasyCrypt syntax tree.
    ///
    fn visit_proc_call(&mut self, pcall: &ProcCall) -> Result<()>;

    ///
    /// Visit a procedure definition in an EasyCrypt syntax tree.
    ///
    fn visit_proc(&mut self, proc: &Proc) -> Result<()>;

    ///
    /// Visit a reference to a variable in an EasyCrypt syntax tree.
    ///
    fn visit_reference(&mut self, reference: &Reference) -> Result<()>;

    ///
    /// Visit a signature of a function or a procedure in an EasyCrypt syntax tree.
    ///
    fn visit_signature(&mut self, signature: &Signature) -> Result<()>;

    ///
    /// Visit an arbitrary statement in an EasyCrypt syntax tree.
    ///
    fn visit_statement(&mut self, statement: &Statement) -> Result<()>;

    ///
    /// Visit an annotated type in an EasyCrypt syntax tree.
    ///
    fn visit_type(&mut self, r#type: &Type) -> Result<()>;

    ///
    /// Visit an unary operation in an EasyCrypt syntax tree.
    ///
    fn visit_unary_op_type(&mut self, op: &UnaryOpType) -> Result<()>;

    ///
    /// Visit a `while` loop.
    ///
    fn visit_while_loop(&mut self, while_loop: &WhileLoop) -> Result<()>;
}
