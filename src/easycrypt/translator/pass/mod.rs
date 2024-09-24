//!
//! Passes that transform EasyCrypt AST
//!

pub mod name_sanitizer;
use anyhow::Result;

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::function::Function;
use crate::easycrypt::syntax::module::Module;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::statement::block::Block;
use crate::easycrypt::syntax::statement::Statement;

///
/// Transforms every expression and statement into something.
///
pub trait SimplePass {
    ///
    /// Transform an arbitrary expression in an EasyCrypt syntax tree.
    ///
    fn transform_expression(&mut self, expression: &Expression) -> Result<Expression>;

    ///
    /// Transform a function definition in an EasyCrypt syntax tree.
    ///
    fn transform_function(&mut self, function: &Function) -> Result<Function>;

    ///
    /// Transform a procedure definition in an EasyCrypt syntax tree.
    ///
    fn transform_proc(&mut self, proc: &Proc) -> Result<Proc>;

    ///
    /// Transform an arbitrary statement in an EasyCrypt syntax tree.
    ///
    fn transform_statement(&mut self, statement: &Statement) -> Result<Statement>;

    ///
    /// Transform a module in an EasyCrypt syntax tree.
    ///
    fn transform_module(&mut self, module: &Module) -> Result<Module>;
}

struct DefaultImpl<'a, T>(&'a mut T)
where
    T: SimplePass;

impl<'a, T> DefaultImpl<'a, T>
where
    T: SimplePass,
{
    fn transform_block(&mut self, block: &Block) -> Result<Block> {
        let mut new_body = Vec::with_capacity(block.statements.len());
        for stmt in &block.statements {
            let new_stmt = self.0.transform_statement(&stmt)?;
            new_body.push(new_stmt)
        }
        Ok(Block {
            statements: new_body,
        })
    }
}

impl<'a, T> SimplePass for DefaultImpl<'a, T>
where
    T: SimplePass,
{
    fn transform_expression(&mut self, expression: &Expression) -> Result<Expression> {
        Ok(expression.clone())
    }

    fn transform_function(&mut self, function: &Function) -> Result<Function> {
        let Function { body, .. } = function;
        let new_body = self.0.transform_expression(body)?;
        Ok(Function {
            body: new_body,
            ..function.clone()
        })
    }

    fn transform_proc(&mut self, proc: &Proc) -> Result<Proc> {
        let Proc { body, .. } = proc;
        Ok(Proc {
            body: self.transform_block(&body)?,
            ..proc.clone()
        })
    }

    fn transform_statement(&mut self, _statement: &Statement) -> Result<Statement> {
        todo!()
    }

    fn transform_module(&mut self, _module: &Module) -> Result<Module> {
        todo!()
    }
}
