//!
//! Transpile a "switch" statement in YUL.
//!

use anyhow::Error;

use crate::easycrypt::path::tracker::PathTracker;
use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::statement::block::Block;
use crate::easycrypt::syntax::statement::if_conditional::IfConditional;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::config::IConfig;
use crate::easycrypt::translator::context::StatementContext;
use crate::easycrypt::translator::expression::context::Context as ExprContext;
use crate::easycrypt::translator::statement::block::Transformed as TransformedBlock;
use crate::easycrypt::translator::statement::Transformed as TransformedStatement;
use crate::easycrypt::translator::Translator;
use era_yul::yul::parser::statement::switch::Switch as YulSwitch;

impl<C> Translator<C>
where
    C: IConfig,
{
    /// Transpile a switch statement in YUL.
    ///
    /// The
    ///
    /// ```
    /// switch expr
    /// case val1 { block1 }
    /// case val2 { block2 }
    /// ...
    /// < default { block_default } >
    /// ```
    ///
    /// Suppose also that:
    ///
    /// - `expr` is translated into statements `expr_statements` and an expression `expr'` ;
    /// - `block1` is translated into `block1'`;
    /// - `block2` is translated into `block2'`;
    /// - optional `block_default` is translated into `block_default'`.
    ///
    /// Then the transpiler emits:
    ///
    /// ```
    /// expr_statements;
    /// let tmp = expr';
    /// if (tmp == val1) { block1' }
    /// else if (tmp == val2) { block2' }
    /// < else { block_default' }>
    /// ```
    pub fn transpile_switch(
        &mut self,
        switch: &YulSwitch<C::Dialect>,
        ctx: &StatementContext,
    ) -> Result<(StatementContext, TransformedStatement), Error> {
        let YulSwitch {
            expression,
            cases,
            default,
            ..
        } = switch;

        let (transpiled_expression, ExprContext { locals, statements }) = self
            .transpile_expression_root(expression, ctx)?
            .expect_expression_and_get()?;
        let mut ctx = ctx.clone();
        ctx.add_locals(locals.iter());

        let (tmp_def, tmp_def_point) = self.new_tmp_definition_here(Type::DEFAULT.clone());
        let tmp_ref = tmp_def_point.reference(self.ec_context.path_tracker.here());
        ctx.add_local(tmp_def);

        let mut result = statements.clone();
        result.push(
            Statement::EAssignment(vec![tmp_ref.clone()], Box::from(transpiled_expression)).clone(),
        );

        for (index, yul_case) in cases.iter().enumerate() {
            let (new_ctx, TransformedBlock { statements }) =
                self.transpile_block(&yul_case.block, &ctx)?;

            let if_stmt = Statement::IfConditional(IfConditional {
                condition: Expression::Binary(
                    BinaryOpType::Eq,
                    Box::from(Expression::Reference(tmp_ref.clone())),
                    Box::from(Self::translate_literal(&yul_case.literal)?),
                ),
                yes: Box::from(Statement::Block(Block { statements })),
                no: None,
            });

            ctx = new_ctx;
            if index == 0 {
                result.push(if_stmt)
            } else if let Statement::IfConditional(ref mut last_if) = result.last_mut().unwrap() {
                last_if.no = Some(Box::from(Statement::Block(Block {
                    statements: vec![if_stmt],
                })))
            }
        }

        if let Some(block) = default {
            let (new_ctx, TransformedBlock { statements }) = self.transpile_block(block, &ctx)?;
            if cases.is_empty() {
                result.extend(statements)
            } else if let Statement::IfConditional(ref mut last_if) = result.last_mut().unwrap() {
                last_if.no = Some(Box::from(Statement::Block(Block { statements })))
            }

            ctx = new_ctx
        }

        Ok((ctx, TransformedStatement::Statements(result)))
    }
}
