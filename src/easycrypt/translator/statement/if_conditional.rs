//!
//! Transpile an "if" conditional statement in YUL.
//!

use std::iter;

use anyhow::Error;

use crate::easycrypt::syntax::statement::block::Block;
use crate::easycrypt::syntax::statement::if_conditional::IfConditional;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::config::IConfig;
use crate::easycrypt::translator::context::StatementContext;
use crate::easycrypt::translator::expression::context::Context as ExprContext;
use crate::easycrypt::translator::statement::block::Transformed as TransformedBlock;
use crate::easycrypt::translator::statement::Transformed;
use crate::easycrypt::translator::Translator;
use crate::yul::path::tracker::PathTracker;
use era_yul::yul::parser::statement::if_conditional::IfConditional as YulIfConditional;

impl<C> Translator<C>
where
    C: IConfig,
{
    ///
    /// Transpile an "if" conditional statement.
    /// Suppose we transpile a YUL statement `if (COND) { BLOCK }`.
    /// - `COND` is transpiled into `COND_STATEMENTS` and `COND`
    /// - `BLOCK` is transpiled into `BLOCK_STATEMENTS`
    ///
    /// The result is then `COND_STATEMENTS; if (COND') { BLOCK_STATEMENTS }`.
    ///
    pub fn transpile_if(
        &mut self,
        conditional: &YulIfConditional<C::Dialect>,
        ctx: &StatementContext,
    ) -> Result<(StatementContext, Transformed), Error> {
        let YulIfConditional {
            condition, block, ..
        } = conditional;

        self.yul_context.path_tracker.enter_if_cond();
        let (
            transpiled_condition,
            ExprContext {
                locals,
                statements: expr_context_statements,
            },
        ) = self
            .transpile_expression_root(condition, ctx)?
            .expect_expression_and_get()?;

        let ctx = ctx.with_locals(&locals);

        self.yul_context.path_tracker.leave();
        self.yul_context.path_tracker.enter_if_then();

        let (
            ctx,
            TransformedBlock {
                statements: block_statements,
            },
        ) = self.transpile_block(block, &ctx)?;

        let transpiled_conditional = IfConditional {
            condition: C::int_to_bool(transpiled_condition)?,
            yes: Box::from(Statement::Block(Block {
                statements: block_statements,
            })),
            no: None,
        };
        self.yul_context.path_tracker.leave();

        let result = Transformed::Statements(
            expr_context_statements
                .iter()
                .chain(iter::once(&Statement::IfConditional(
                    transpiled_conditional,
                )))
                .cloned()
                .collect(),
        );
        Ok((ctx, result))
    }
}
