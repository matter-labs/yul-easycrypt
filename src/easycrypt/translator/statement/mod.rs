//!
//! Transpilation of arbitrary YUL statements.
//!

pub mod assignment;
pub mod block;
pub mod for_loop;
pub mod if_conditional;
pub mod switch;
pub mod variable_declaration;

use anyhow::Error;
use era_yul::yul::parser::statement::Statement as YulStatement;

use crate::easycrypt::syntax::function::Function;
use crate::easycrypt::syntax::module::definition::TopDefinition;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::statement::Statement;

use super::config::IConfig;
use super::context::StatementContext;
use super::function;

pub enum Transformed {
    Statements(Vec<Statement>),
    Function(Function),
    Proc(Proc),
}

impl Transformed {
    pub fn as_statements(&self) -> Option<&Vec<Statement>> {
        if let Self::Statements(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl<C> super::Translator<C>
where
    C: IConfig,
{
    /// Transpile an arbitrary YUL statement.
    pub fn transpile_statement(
        &mut self,
        stmt: &YulStatement<C::Dialect>,
        ctx: &StatementContext,
    ) -> Result<(StatementContext, Transformed), Error> {
        match stmt {
            YulStatement::Object(_) => {
                unimplemented!("Unable to translate a YUL statement with an 'object'")
            }
            YulStatement::Code(_) => {
                unimplemented!("Unable to translate a YUL statement with a 'code'")
            }
            YulStatement::Block(block) => {
                let (ctx, transpiled) = self.transpile_block(block, ctx)?;
                Ok((ctx, Transformed::Statements(transpiled.statements)))
            }
            YulStatement::Expression(_) => todo!(),
            YulStatement::FunctionDefinition(fd) => {
                let translation_result = self.translate_function_definition(fd)?;
                match translation_result {
                    function::Translated::Function(ec_function) => {
                        self.result
                            .add_def(TopDefinition::Function(ec_function.clone()));
                        Ok((ctx.clone(), Transformed::Function(ec_function)))
                    }
                    function::Translated::Proc(ec_procedure) => {
                        self.result
                            .add_def(TopDefinition::Proc(ec_procedure.clone()));
                        Ok((ctx.clone(), Transformed::Proc(ec_procedure)))
                    }
                }
            }
            YulStatement::VariableDeclaration(vd) => self.transpile_variable_declaration(vd, ctx),
            YulStatement::Assignment(assignment) => self.transpile_assignment(assignment, ctx),
            YulStatement::IfConditional(if_conditional) => self.transpile_if(if_conditional, ctx),
            YulStatement::Switch(_)
            | YulStatement::ForLoop(_)
            | YulStatement::Continue(_)
            | YulStatement::Break(_)
            | YulStatement::Leave(_) => {
                Ok((StatementContext::new(), Transformed::Statements(vec![])))
            }
        }
        //     YulStatement::Expression(expr) => match self.transpile_expression_root(expr, ctx)? {
        //         super::expression::Transformed::Expression(Expression::Reference(_), ectx) => Ok((
        //             ctx.add_locals(&ectx.locals),
        //             Transformed::Statements(ectx.assignments.to_vec()),
        //         )),
        //         super::expression::Transformed::Expression(result, ectx) => Ok((
        //             ctx.add_locals(&ectx.locals),
        //             Transformed::Statements(
        //                 ectx.assignments
        //                     .iter()
        //                     .chain(iter::once(&Statement::Expression(result)))
        //                     .cloned()
        //                     .collect(),
        //             ),
        //         )),
        //         super::expression::Transformed::Statements(statements, ectx, ctx) => {
        //             let result = ectx
        //                 .assignments
        //                 .iter()
        //                 .chain(statements.iter())
        //                 .cloned()
        //                 .collect();
        //             Ok((
        //                 ctx.add_locals(&ectx.locals),
        //                 Transformed::Statements(result),
        //             ))
        //         }
        //     },
        //     YulStatement::VariableDeclaration(vd) => self.transpile_variable_declaration(vd, ctx),
        //     YulStatement::Assignment(assignment) => self.transpile_assignment(assignment, ctx),
        //     YulStatement::IfConditional(conditional) => self.transpile_if(conditional, ctx),
        //     YulStatement::Switch(switch) => self.transpile_switch(switch, ctx),
        //     YulStatement::ForLoop(for_loop) => self.transpile_for_loop(for_loop, ctx),
        //     YulStatement::Continue(_) => {
        //         anyhow::bail!("The `continue` statement is not supported.")
        //     }
        //     YulStatement::Break(_) => anyhow::bail!("The `break` statement is not supported."),
        //     YulStatement::Leave(_) => anyhow::bail!("The `leave` statement is not supported."),
        // }
    }
}
