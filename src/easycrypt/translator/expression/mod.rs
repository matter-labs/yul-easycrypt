//!
//! Transpilation of YUL expressions.
//!

pub mod context;
pub mod function_call;
pub mod identifier;
pub mod literal;

use anyhow::Error;
use anyhow::Result;

use era_yul::yul::parser::statement::expression::function_call::FunctionCall as YulFunctionCall;
use era_yul::yul::parser::statement::expression::Expression as YulExpression;

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::expression::context::Context as ExprContext;

use super::config::IConfig;
use super::context::StatementContext;

#[derive(Debug)]
pub enum Transformed {
    Expression(Expression, ExprContext),
    Statements(Vec<Statement>, ExprContext, StatementContext),
}

impl Transformed {
    pub fn expect_expression_and_get(self) -> Result<(Expression, ExprContext)> {
        if let Self::Expression(expr, ectx) = self {
            Ok((expr, ectx))
        } else {
            anyhow::bail!(
                format!("{} \n {:#?}", Self::MSG_EXPECTED_EXPRESSION_RESULT, &self).to_string()
            )
        }
    }

    pub const MSG_EXPECTED_EXPRESSION_RESULT : &'static str = "Malformed YUL: all expressions in an expression list are supposed to be transpiled into expressions.";
}

impl<C> super::Translator<C>
where
    C: IConfig,
{
    ///
    /// Transpile multiple YUL expressions accumulating the context.
    ///
    pub fn transpile_expression_list(
        &mut self,
        list: &[YulExpression],
        ctx: &StatementContext,
        ectx: &ExprContext,
    ) -> Result<(Vec<Expression>, ExprContext), Error> {
        let mut ectx: ExprContext = ectx.clone();
        let mut result: Vec<Expression> = Vec::new();

        for expr in list {
            if let Transformed::Expression(e, new_ectx) =
                self.transpile_expression(expr, ctx, &ectx, false)?
            {
                ectx = new_ectx;
                result.push(e);
            } else {
                anyhow::bail!(Transformed::MSG_EXPECTED_EXPRESSION_RESULT)
            }
        }
        Ok((result, ectx))
    }

    ///
    /// Transpile an arbitrary YUL expression.
    ///
    fn transpile_expression(
        &mut self,
        expr: &YulExpression,
        ctx: &StatementContext,
        ectx: &ExprContext,
        is_root: bool,
    ) -> Result<Transformed> {
        match expr {
            YulExpression::FunctionCall(YulFunctionCall {
                location: _,
                name,
                arguments,
            }) => self.transpile_function_call(name, arguments, ctx, ectx, is_root),

            YulExpression::Identifier(ident) => {
                let reference = self.transpile_identifier_reference(ident)?;
                Ok(Transformed::Expression(
                    Expression::Reference(reference),
                    ectx.clone(),
                ))
            }
            YulExpression::Literal(lit) => Ok(Transformed::Expression(
                Self::translate_literal(lit)?,
                ectx.clone(),
            )),
        }
    }

    ///
    /// Transpile a YUL expression that is not a subexpression of any other expression.
    ///
    pub fn transpile_expression_root(
        &mut self,
        expr: &YulExpression,
        ctx: &StatementContext,
    ) -> Result<Transformed> {
        self.transpile_expression(expr, ctx, &ExprContext::new(), true)
    }
}
