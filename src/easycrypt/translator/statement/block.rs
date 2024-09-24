//!
//! Transpilation of YUL blocks of statements.
//!

use anyhow::Error;

use crate::easycrypt::syntax::module::definition::TopDefinition;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::config::IConfig;
use crate::easycrypt::translator::context::StatementContext;
use crate::easycrypt::translator::statement::Transformed as TransformedStatement;
use crate::easycrypt::translator::Translator;
use crate::yul::path::tracker::PathTracker;
use era_yul::yul::parser::statement::block::Block as YulBlock;

#[derive(Default)]
pub struct Transformed {
    pub statements: Vec<Statement>,
}

impl<C> Translator<C>
where
    C: IConfig,
{
    ///
    /// Transpile an arbitrary YUL block.
    ///
    pub fn transpile_block(
        &mut self,
        block: &YulBlock<C::Dialect>,
        ctx: &StatementContext,
    ) -> Result<(StatementContext, Transformed), Error> {
        let mut current_context = ctx.clone();
        let mut result = Transformed::default();

        self.yul_context.path_tracker.enter_block();
        for stmt in block.statements.iter() {
            let (ctx, translated) = self.transpile_statement(stmt, &current_context)?;
            match translated {
                TransformedStatement::Statements(stmts) => {
                    result.statements.extend(stmts);
                    current_context = ctx;
                }
                TransformedStatement::Function(fun) => {
                    self.result.add_def(TopDefinition::Function(fun))
                }
                TransformedStatement::Proc(proc) => self.result.add_def(TopDefinition::Proc(proc)),
            };
        }
        self.yul_context.path_tracker.leave();
        Ok((current_context, result))
    }
}
