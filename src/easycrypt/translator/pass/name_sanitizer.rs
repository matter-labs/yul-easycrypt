use super::{DefaultImpl, SimplePass};
use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::expression::call::FunctionCall;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::function::Function;
use crate::easycrypt::syntax::module::definition::TopDefinition;
use crate::easycrypt::syntax::module::Module;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::statement::block::Block;
use crate::easycrypt::syntax::statement::call::ProcCall;
use crate::easycrypt::syntax::statement::if_conditional::IfConditional;
use crate::easycrypt::syntax::statement::while_loop::WhileLoop;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::syntax::{reference::Reference, signature::Signature};
use anyhow::Result;

///
/// Taken from Section "2.1 Lexical Categories" of the Easycrypt Reference Manual
/// See https://www.easycrypt.info/easycrypt-doc/refman.pdf
///
const KEYWORDS: &[&str] = &[
    "abbrev",
    "abort",
    "abstract",
    "admit",
    "admitted",
    "algebra",
    "alias",
    "apply",
    "as",
    "assert",
    "assumption",
    "async",
    "auto",
    "axiom",
    "axiomatized",
    "beta",
    "by",
    "byequiv",
    "byphoare",
    "bypr",
    "call",
    "case",
    "cfold",
    "change",
    "class",
    "clear",
    "clone",
    "congr",
    "conseq",
    "const",
    "cut",
    "debug",
    "declare",
    "delta",
    "do",
    "done",
    "dump",
    "eager",
    "elif",
    "elim",
    "else",
    "end",
    "equiv",
    "eta",
    "exact",
    "exfalso",
    "exists",
    "expect",
    "export",
    "fel",
    "field",
    "fieldeq",
    "first",
    "fission",
    "forall",
    "fun",
    "fusion",
    "glob",
    "goal",
    "have",
    "hint",
    "hoare",
    "idtac",
    "if",
    "import",
    "in",
    "include",
    "inductive",
    "inline",
    "instance",
    "iota",
    "islossless",
    "kill",
    "last",
    "left",
    "lemma",
    "let",
    "local",
    "logic",
    "modpath",
    "module",
    "move",
    "nosmt",
    "notation",
    "of",
    "op",
    "phoare",
    "pose",
    "Pr",
    "pr_bounded",
    "pragma",
    "pred",
    "print",
    "proc",
    "progress",
    "proof",
    "prover",
    "qed",
    "rcondf",
    "rcondt",
    "realize",
    "reflexivity",
    "remove",
    "rename",
    "replace",
    "require",
    "res",
    "return",
    "rewrite",
    "right",
    "ring",
    "ringeq",
    "rnd",
    "rwnormal",
    "search",
    "section",
    "Self",
    "seq",
    "sim",
    "simplify",
    "skip",
    "smt",
    "solve",
    "sp",
    "split",
    "splitwhile",
    "strict",
    "subst",
    "suff",
    "swap",
    "symmetry",
    "then",
    "theory",
    "time",
    "timeout",
    "Top",
    "transitivity",
    "trivial",
    "try",
    "type",
    "undo",
    "unroll",
    "var",
    "while",
    "why3",
    "with",
    "wlog",
    "wp",
    "zeta",
];

fn sanitize_identifier(identifier: &str) -> String {
    let mut result = identifier.replace('$', "_");
    let starts_with_uppercase = identifier
        .chars()
        .next()
        .map_or(false, |c| c.is_uppercase());

    if starts_with_uppercase || KEYWORDS.contains(&identifier) {
        result.insert(0, '_');
    }
    result
}

pub struct NameSanitizer {}

impl NameSanitizer {
    pub fn new() -> Self {
        Self {}
    }

    fn transform_reference(reference: &Reference) -> Reference {
        let Reference { identifier, path } = reference;
        Reference {
            identifier: sanitize_identifier(&identifier),
            path: path.clone(),
        }
    }
    fn transform_definition(definition: &Definition) -> Definition {
        let Definition { identifier, r#type } = definition;
        Definition {
            identifier: sanitize_identifier(&identifier),
            r#type: r#type.clone(),
        }
    }

    fn transform_signature(signature: &Signature) -> Signature {
        let Signature {
            formal_parameters,
            return_type,
            kind,
        } = signature;
        let new_formal_parameters = formal_parameters
            .iter()
            .map(Self::transform_definition)
            .collect();
        Signature {
            formal_parameters: new_formal_parameters,
            return_type: return_type.clone(),
            kind: kind.clone(),
        }
    }

    fn transform_expressions(&mut self, expressions: &Vec<Expression>) -> Result<Vec<Expression>> {
        let mut result = Vec::with_capacity(expressions.len());
        for expression in expressions {
            result.push(self.transform_expression(&expression)?);
        }
        Ok(result)
    }
    fn transform_statements(&mut self, statements: &Vec<Statement>) -> Result<Vec<Statement>> {
        let mut result = Vec::with_capacity(statements.len());
        for statement in statements {
            result.push(self.transform_statement(&statement)?);
        }
        Ok(result)
    }
}

impl SimplePass for NameSanitizer {
    fn transform_expression(&mut self, expression: &Expression) -> Result<Expression> {
        match expression {
            Expression::Unary(op_type, expr) => {
                let inner = self.transform_expression(expr)?;
                Ok(Expression::Unary(op_type.clone(), Box::new(inner)))
            }
            Expression::Binary(op_type, left, right) => {
                let inner_left = self.transform_expression(left)?;
                let inner_right = self.transform_expression(right)?;
                Ok(Expression::Binary(
                    op_type.clone(),
                    Box::new(inner_left),
                    Box::new(inner_right),
                ))
            }
            Expression::ECall(FunctionCall { target, arguments }) => {
                let new_arguments = self.transform_expressions(arguments)?;
                let new_target = Self::transform_reference(target);
                Ok(Expression::ECall(FunctionCall {
                    target: new_target,
                    arguments: new_arguments,
                }))
            }
            Expression::Literal(lit) => Ok(Expression::Literal(lit.clone())),
            Expression::Reference(reference) => {
                Ok(Expression::Reference(Self::transform_reference(reference)))
            }
            Expression::Tuple(expressions) => {
                Ok(Expression::Tuple(self.transform_expressions(expressions)?))
            }
        }
    }

    fn transform_function(&mut self, function: &Function) -> Result<Function> {
        let mut transformed = DefaultImpl(self).transform_function(function)?;
        transformed.signature = Self::transform_signature(&function.signature);
        transformed.name = sanitize_identifier(&function.name);
        Ok(transformed)
    }

    fn transform_proc(&mut self, proc: &Proc) -> Result<Proc> {
        let mut transformed = DefaultImpl(self).transform_proc(proc)?;
        transformed.signature = Self::transform_signature(&proc.signature);
        transformed.name = sanitize_identifier(&proc.name);
        transformed.locals = proc.locals.iter().map(Self::transform_definition).collect();
        Ok(transformed)
    }

    fn transform_statement(&mut self, statement: &Statement) -> Result<Statement> {
        match statement {
            Statement::Expression(expression) => Ok(Statement::Expression(
                self.transform_expression(expression)?,
            )),
            Statement::Block(block) => {
                let new_statements = self.transform_statements(&block.statements)?;
                Ok(Statement::Block(Block {
                    statements: new_statements,
                }))
            }
            Statement::IfConditional(IfConditional { condition, yes, no }) => {
                let new_condition = self.transform_expression(condition)?;
                let new_yes = Box::new(self.transform_statement(yes)?);
                let new_no = {
                    match no {
                        Some(no) => Some(Box::new(self.transform_statement(no)?)),
                        None => None,
                    }
                };

                Ok(Statement::IfConditional(IfConditional {
                    condition: new_condition,
                    yes: new_yes,
                    no: new_no,
                }))
            }
            Statement::EAssignment(references, expression) => {
                let new_references = references.iter().map(Self::transform_reference).collect();
                let new_expression = self.transform_expression(expression)?;
                Ok(Statement::EAssignment(
                    new_references,
                    Box::new(new_expression),
                ))
            }
            Statement::PAssignment(references, ProcCall { target, arguments }) => {
                let new_references = references.iter().map(Self::transform_reference).collect();
                let new_target = Self::transform_reference(target);
                let new_arguments = self.transform_expressions(arguments)?;
                Ok(Statement::PAssignment(
                    new_references,
                    ProcCall {
                        target: new_target,
                        arguments: new_arguments,
                    },
                ))
            }
            Statement::Return(expression) => {
                Ok(Statement::Return(self.transform_expression(expression)?))
            }

            Statement::WhileLoop(WhileLoop { condition, body }) => {
                let new_condition = self.transform_expression(condition)?;
                let new_body = Box::new(self.transform_statement(body)?);
                Ok(Statement::WhileLoop(WhileLoop {
                    condition: new_condition,
                    body: new_body,
                }))
            }
        }
    }

    fn transform_module(&mut self, module: &Module) -> Result<Module> {
        let new_name = module.name.as_deref().map(sanitize_identifier);
        let mut result = Module::new(new_name);
        for (_, definition) in module.definitions.iter() {
            match definition {
                TopDefinition::Proc(proc) => {
                    result.add_def(TopDefinition::Proc(self.transform_proc(proc)?));
                }
                TopDefinition::Function(function) => {
                    result.add_def(TopDefinition::Function(self.transform_function(function)?));
                }
            }
        }

        Ok(result)
    }
}
