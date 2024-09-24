//!
//! Transpilation of Yul functions.
//!

use anyhow::Result;
use std::iter::once;

use crate::easycrypt::path::tracker::PathTracker as _;
use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::function::Function;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::signature::Signature;
use crate::easycrypt::syntax::signature::SignatureKind;
use crate::easycrypt::syntax::statement::block::Block;
use crate::easycrypt::syntax::statement::Statement;
use crate::yul::path::full_name::FullName;
use crate::yul::path::tracker::PathTracker as _;
use era_yul::yul::parser::statement::function_definition::FunctionDefinition;

use super::config::IConfig;
use super::context::StatementContext;
use super::state::definition_location::DefinitionLocation;
use super::Translator;

pub enum Translated {
    Function(Function),
    Proc(Proc),
}

fn contains<T, I>(mut iter: I, element: &T) -> bool
where
    T: PartialEq,
    I: Iterator<Item = T>,
{
    iter.any(|item| item == *element)
}

impl<C> Translator<C>
where
    C: IConfig,
{
    fn locals<'a, 'b, 'c>(
        ec_parameters: impl Iterator<Item = &'a Definition>,
        ec_return_values: impl Iterator<Item = &'b Definition>,
        context_locals: impl Iterator<Item = &'c Definition>,
    ) -> Vec<Definition> {
        let return_values: Vec<Definition> = ec_return_values.cloned().collect();
        let mut filtered_params: Vec<_> = ec_parameters
            .filter(|def| contains(return_values.iter(), def))
            .cloned()
            .collect();
        filtered_params.extend(return_values);
        filtered_params.extend(context_locals.cloned());
        filtered_params
    }
    ///
    /// Transpile an arbitrary YUL function into an EasyCrypt function or procedure.
    ///
    pub fn translate_function_definition(
        &mut self,
        fd: &FunctionDefinition<C::Dialect>,
    ) -> Result<Translated> {
        let FunctionDefinition {
            identifier,
            arguments,
            result,
            body,
            ..
        } = fd;

        self.preimages.insert(
            DefinitionLocation {
                identifier: identifier.to_string(),
                path: self.ec_context.path_tracker.here().clone(),
            },
            FullName::custom(identifier, self.get_yul_path()),
        );

        self.yul_context.path_tracker.enter_function(identifier);
        self.ec_context.path_tracker.enter_procedure(identifier);

        let arguments: Vec<(Definition, DefinitionLocation)> =
            self.bindings_to_definitions(arguments);
        let result_vars: Vec<(Definition, DefinitionLocation)> =
            self.bindings_to_definitions(result);
        let return_type: Type =
            Type::of_types(result_vars.iter().map(|(def, _)| def.get_effective_type()));

        let (ctx, ec_block) = self.transpile_block(body, &StatementContext::new())?;

        let formal_parameters: Vec<Definition> =
            arguments.iter().map(|(def, _)| def).cloned().collect();

        let locals: Vec<Definition> = Self::locals(
            formal_parameters.iter().by_ref(),
            result_vars.iter().map(|x| &x.0),
            ctx.locals.iter(),
        );

        let signature = Signature {
            formal_parameters,
            return_type,
            kind: SignatureKind::Proc,
        };

        let body: Vec<Statement> = {
            if !signature.returns_unit() {
                let return_statement = Statement::Return(Expression::pack_tuple(
                    &result_vars
                        .iter()
                        .map(|(def, _)| Expression::Reference(def.reference()))
                        .collect::<Vec<_>>(),
                ));
                ec_block
                    .statements
                    .into_iter()
                    .chain(once(return_statement))
                    .collect()
            } else {
                ec_block.statements
            }
        };

        self.yul_context.path_tracker.leave();
        self.ec_context.path_tracker.leave();

        Ok(Translated::Proc(Proc {
            name: identifier.to_string(),
            signature,
            locals,
            body: Block { statements: body },
        }))

        // let full_name = self.create_full_name(identifier);
        // self.functions_stack.push(full_name.clone());
        // let definition = self.definitions.get(&full_name).unwrap();

        // let kind = definition.kind.clone();
        // self.tracker.enter_function(identifier);

        // let formal_parameters = self.all_formal_parameters(arguments, definition);
        // let result_vars = self.all_result_variables(result, definition);
        // let return_type: Type = Type::type_of_definitions(result_vars.as_slice());

        // let (ctx, ec_block) = self.transpile_block(body, &ctx.clear_locals())?;
        // match kind {
        //     // FIXME ugly
        //     Kind::Function(_) => {
        //         match &ec_block.statements[0] {
        //             Statement::EAssignment(_, expr) =>  {
        //                 self.translate_to_function(formal_parameters, return_type, &ctx, identifier, &full_name, expr)
        //             },
        //             _ => anyhow::bail!("Attempt to translate a YUL function into EasyCrypt function, but only translating to procedure is possible."),

        //         }
        //     },
        //     Kind::Proc(_) => {
        //         self.translate_to_procedure(
        //             &formal_parameters,
        //             return_type,
        //             result_vars,
        //             ec_block,
        //             ctx,
        //             &full_name,
        //             identifier,
        //         )
        //     }
        //     _ => anyhow::bail!("Malformed collection of definitions"),
        // }
    }

    // fn translate_to_procedure(
    //     &mut self,
    //     formal_parameters: &[(Definition, Type)],
    //     return_type: Type,
    //     result_vars: Vec<Definition>,
    //     ec_block: TransformedBlock,
    //     ctx: Context,
    //     yul_name: &FullName,
    //     identifier: &str,
    // ) -> Result<(Context, Translated), Error> {
    //     let signature = Signature {
    //         formal_parameters: formal_parameters.to_vec(),
    //         return_type,
    //         kind: SignatureKind::Proc,
    //     };
    //     let statements = if signature.return_type != Type::Unit {
    //         let return_statement = Statement::Return(Expression::pack_tuple(
    //             &result_vars
    //                 .iter()
    //                 .map(|d| Expression::Reference(d.reference()))
    //                 .collect::<Vec<_>>(),
    //         ));
    //         ec_block
    //             .statements
    //             .iter()
    //             .chain(iter::once(&return_statement))
    //             .cloned()
    //             .collect()
    //     } else {
    //         ec_block.statements
    //     };
    //     let locals = result_vars
    //         .iter()
    //         .filter(|def| !formal_parameters.iter().any(|param| param.0 == **def))
    //         .chain(ctx.locals.iter())
    //         .cloned()
    //         .collect();
    //     self.tracker.leave();
    //     self.functions_stack.pop();
    //     Ok((
    //         ctx.clone(),
    //         Translated::Proc(Proc {
    //             name: ProcName {
    //                 name: identifier.to_string(),
    //                 module: None,
    //                 yul_name: Some(yul_name.clone()),
    //             },
    //             signature,
    //             locals,
    //             body: Block { statements },
    //         }),
    //     ))
    // }

    // fn translate_to_function(
    //     &mut self,
    //     formal_parameters: Vec<(Definition, Type)>,
    //     return_type: Type,
    //     ctx: &Context,
    //     identifier: &str,
    //     yul_name: &FullName,
    //     body_expr: &Expression,
    // ) -> Result<(Context, Translated), Error> {
    //     let signature = Signature {
    //         formal_parameters,
    //         return_type,
    //         kind: SignatureKind::Function,
    //     };
    //     self.tracker.leave();
    //     self.functions_stack.pop();
    //     Ok((
    //         ctx.clone(),
    //         Translated::Function(Function {
    //             name: FunctionName {
    //                 name: identifier.to_string(),
    //                 module: None,
    //                 yul_name: Some(yul_name.clone()),
    //             },
    //             signature,
    //             body: body_expr.clone(),
    //         }),
    //     ))
    // }
    //
    // fn all_formal_parameters(
    //     &mut self,
    //     yul_parameters: impl Iterator<Item = YulIdentifier>,
    // ) -> Vec<(Definition, DefinitionPoint)> {
    //     yul_parameters
    //         .map(|ident| self.transpile_formal_parameter(&ident))
    //         .collect::<Vec<_>>()
    // }

    // fn transpile_formal_parameter(
    //     &mut self,
    //     ident: &YulIdentifier,
    // ) -> (Definition, DefinitionPoint) {
    //     let typ = ident
    //         .r#type
    //         .clone()
    //         .map(|t| Self::translate_type(&t).unwrap())
    //         .unwrap_or(Type::DEFAULT.clone());
    //     self.new_definition_here(&ident.inner, typ.clone(), &ident.inner)
    // }
}
