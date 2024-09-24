//!
//! Transpilation of YUL function calls.
//!

use anyhow::anyhow;
use anyhow::Result;

use crate::easycrypt::path::tracker::PathTracker;
use crate::easycrypt::syntax::expression::call::FunctionCall;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::statement::call::ProcCall;
use crate::easycrypt::translator::config::IConfig;
use crate::easycrypt::translator::expression::Transformed;
use crate::easycrypt::translator::state::definition_info::Builtin;
use crate::easycrypt::translator::state::definition_info::Custom;
use crate::easycrypt::translator::state::definition_info::DefinitionInfo;
use crate::easycrypt::translator::state::definition_info::Description;
use crate::easycrypt::translator::state::definition_info::KindSpecific;
use crate::easycrypt::translator::Translator;

use era_yul::yul::parser::statement::expression::function_call::name::Name as YulName;
use era_yul::yul::parser::statement::expression::Expression as YulExpression;

use crate::easycrypt::translator::context::StatementContext;
use crate::easycrypt::translator::expression::context::Context as ExprContext;

impl<C> Translator<C>
where
    C: IConfig,
{
    ///
    /// Transpile a function call in YUL into either EasyCrypt procedure or function.
    ///
    pub fn transpile_function_call(
        &mut self,
        name: &YulName,
        yul_arguments: &[YulExpression],
        ctx: &StatementContext,
        ectx: &ExprContext,
        is_root: bool,
    ) -> Result<Transformed> {
        let definition_info: DefinitionInfo = self
            .get_ec_definition_info(name.clone())
            .ok_or(anyhow!(
                "Internal error: function call to an undefined function {:?}",
                &name
            ))?
            .clone();
        let transformed = match &definition_info.description {
            Description::Builtin(Builtin::UnaryOperation(op_kind)) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                anyhow::ensure!(
                    arguments.len() == 1,
                    "Expected exactly two arguments for the function {:?}",
                    name
                );
                Ok(Transformed::Expression(
                    Expression::Unary(op_kind.clone(), Box::new(arguments[0].clone())),
                    ectx,
                ))
            }

            Description::Builtin(Builtin::BinaryOperation(op_kind)) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                anyhow::ensure!(
                    arguments.len() == 2,
                    "Expected exactly two arguments for the function {:?}",
                    name
                );
                Ok(Transformed::Expression(
                    Expression::Binary(
                        op_kind.clone(),
                        Box::new(arguments[0].clone()),
                        Box::new(arguments[1].clone()),
                    ),
                    ectx,
                ))
            }
            Description::Custom(Custom {
                specific: KindSpecific::Function,
                location,
            }) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                let target = location.reference(self.ec_context.path_tracker.here());
                Ok(Transformed::Expression(
                    Expression::ECall(FunctionCall { target, arguments }),
                    ectx,
                ))
            }
            Description::Custom(Custom {
                specific: KindSpecific::Proc,
                location,
            }) => {
                let (arguments, mut ectx) =
                    self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                let returns_unit = matches!(&definition_info.r#type, Type::Arrow(_, ret_type) if **ret_type == Type::Unit);

                let return_vars = {
                    if returns_unit {
                        vec![]
                    } else {
                        let tmp_def = self.new_tmp_definition_here(definition_info.r#type);
                        ectx.locals.push(tmp_def.0.clone());
                        vec![tmp_def.0.reference()]
                    }
                };

                let mut new_ectx = ectx;

                new_ectx.add_multiple_assignment(
                    &return_vars,
                    ProcCall {
                        target: location.reference(self.ec_context.path_tracker.here()),
                        arguments,
                    },
                );

                if return_vars.is_empty() && is_root {
                    Ok(Transformed::Statements(vec![], new_ectx, ctx.clone()))
                } else {
                    Ok(Transformed::Expression(
                        Expression::Reference(return_vars[0].clone()),
                        new_ectx,
                    ))
                }
            }

            Description::Custom(Custom {
                specific: KindSpecific::Variable,
                ..
            }) => {
                anyhow::bail!("Internal error: Attempt to call a variable")
            }
        };
        transformed
    }
}
