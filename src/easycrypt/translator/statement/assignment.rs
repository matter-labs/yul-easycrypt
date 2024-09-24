//!
//! Transpile YUL assignments.
//!

use anyhow::Result;

use crate::easycrypt::translator::config::IConfig;
use crate::easycrypt::translator::context::StatementContext;
use crate::easycrypt::translator::Translator;

use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::expression::context::Context as ExprContext;
use crate::easycrypt::translator::statement::Transformed;
use era_yul::yul::parser::identifier::Identifier as YulIdentifier;
use era_yul::yul::parser::statement::assignment::Assignment as YulAssignment;
use era_yul::yul::parser::statement::expression::Expression as YulExpression;

impl<C> Translator<C>
where
    C: IConfig,
{
    fn transpile_assignment_aux(
        &mut self,
        bindings: &[YulIdentifier],
        initializer: &YulExpression,
        ctx: &StatementContext,
    ) -> Result<(StatementContext, Transformed)> {
        let references = {
            let idents = bindings.iter().cloned();
            let mut result = Vec::new();
            for i in idents {
                result.push(self.transpile_identifier_reference(&i)?)
            }
            result
        };

        let (
            new_rhs,
            ExprContext {
                statements: mut assignments,
                locals,
            },
        ) = self
            .transpile_expression_root(initializer, ctx)?
            .expect_expression_and_get()?;

        let ec_assignment = Statement::EAssignment(references, Box::new(new_rhs));

        assignments.push(ec_assignment);

        Ok((
            ctx.with_locals(locals.iter()),
            Transformed::Statements(assignments),
        ))
    }

    ///
    /// Transpile a YUL assignment.
    ///
    pub fn transpile_assignment(
        &mut self,
        assignment: &YulAssignment,
        ctx: &StatementContext,
    ) -> Result<(StatementContext, Transformed)> {
        let YulAssignment {
            location: _,
            bindings,
            initializer,
        } = assignment;
        self.transpile_assignment_aux(bindings, initializer, ctx)
    }
}
