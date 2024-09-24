//!
//! Transpilation of YUL variable declarations.
//!

use anyhow::Error;

use super::Transformed;
use crate::easycrypt::translator::config::IConfig;
use crate::easycrypt::translator::context::StatementContext;
use crate::easycrypt::translator::Translator;
use era_yul::yul::parser::statement::assignment::Assignment as YulAssignment;
use era_yul::yul::parser::statement::variable_declaration::VariableDeclaration;

impl<C> Translator<C>
where
    C: IConfig,
{
    ///
    /// Transpiles `var x,y,... = expr` or `var x,y` as follows:
    /// 1. Transform expression `expr`. This may produce additional statements
    /// and new temporary locals when `expr` contains function calls that are
    /// transpiled into procedure calls: each procedure call should be a
    /// distinct statement. All of them should be added to the context `ctx`.
    /// 2. Add `x,y,...` to the list of locals in context `ctx`
    /// 3. Return an assignment, if there was an expression on the right hand side.
    ///
    pub fn transpile_variable_declaration(
        &mut self,
        vd: &VariableDeclaration,
        ctx: &StatementContext,
    ) -> Result<(StatementContext, Transformed), Error> {
        let VariableDeclaration {
            location,
            bindings,
            expression,
        } = vd;
        let definitions = self.bindings_to_definitions(bindings);

        let ctx = ctx.with_locals(definitions.iter().map(|d| &d.0));
        if let Some(initializer) = expression {
            let equivalent_assignment = YulAssignment {
                location: *location,
                initializer: initializer.clone(),
                bindings: bindings.to_vec(),
            };

            self.transpile_assignment(&equivalent_assignment, &ctx)
        } else {
            Ok((ctx, Transformed::Statements(vec![])))
        }
    }
}
