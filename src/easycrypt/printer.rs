//!
//! EasyCrypt AST pretty printer
//!

use std::collections::HashMap;

use anyhow::Result;
use era_yul::util::printer::print_list_comma_separated;

use super::syntax::definition::Definition;
use super::syntax::expression::binary::BinaryOpType;
use super::syntax::expression::call::FunctionCall;
use super::syntax::expression::literal::integer::IntegerLiteral;
use super::syntax::expression::literal::Literal;
use super::syntax::expression::unary::UnaryOpType;
use super::syntax::expression::Expression;
use super::syntax::function::Function;
use super::syntax::module::definition::TopDefinition;
use super::syntax::module::Module;
use super::syntax::proc::Proc;
use super::syntax::r#type::Type;
use super::syntax::reference::Reference;
use super::syntax::signature::Signature;
use super::syntax::signature::SignatureKind;
use super::syntax::statement::block::Block;
use super::syntax::statement::call::ProcCall;
use super::syntax::statement::if_conditional::IfConditional;
use super::syntax::statement::Statement;

use super::syntax::statement::while_loop::WhileLoop;
use super::visitor::Visitor;
use crate::util::iter::group_by;
use crate::WritePrinter;
use era_yul::util::printer::IPrinter;

fn statement_followed_by_semicolon(statement: &Statement) -> bool {
    match &statement {
        Statement::Block(_) | Statement::IfConditional(_) => false,
        Statement::Expression(_) | Statement::EAssignment(_, _) | Statement::PAssignment(_, _) => {
            true
        }
        Statement::Return(_) => true,
        Statement::WhileLoop(_) => false,
    }
}

pub struct ECPrinter<W>
where
    W: std::fmt::Write,
{
    pub printer: WritePrinter<W>,
}

impl<W> ECPrinter<W>
where
    W: std::fmt::Write,
{
    pub fn new(writer: W) -> Self {
        Self {
            printer: WritePrinter::new(writer),
        }
    }
    fn visit_locals(&mut self, locals: &Vec<Definition>) -> Result<()> {
        let grouped: HashMap<Type, Vec<Definition>> =
            group_by(&locals, |def| def.get_effective_type());
        for (typ, variables) in grouped {
            self.print("var ")?;
            print_list_comma_separated(variables.iter().map(|def| def.identifier.as_str()), self)?;
            self.print(" :")?;
            self.visit_type(&typ)?;
            self.println(";")?;
        }
        Ok(())
    }
}

impl<W> IPrinter for ECPrinter<W>
where
    W: std::fmt::Write,
{
    fn print(&mut self, s: &str) -> Result<()> {
        <WritePrinter<W> as IPrinter>::print(&mut self.printer, s)
    }

    fn println(&mut self, s: &str) -> Result<()> {
        <WritePrinter<W> as IPrinter>::println(&mut self.printer, s)
    }

    fn increase_indent(&mut self) -> Result<()> {
        <WritePrinter<W> as IPrinter>::increase_indent(&mut self.printer)
    }

    fn decrease_indent(&mut self) -> Result<()> {
        <WritePrinter<W> as IPrinter>::decrease_indent(&mut self.printer)
    }
}

impl<W> Visitor for ECPrinter<W>
where
    W: std::fmt::Write,
{
    fn visit_binary_op_type(&mut self, op: &BinaryOpType) -> Result<()> {
        self.print(match op {
            BinaryOpType::Add => "+",
            BinaryOpType::Sub => "-",
            BinaryOpType::Mul => "*",
            BinaryOpType::Mod => "%%",
            BinaryOpType::And => "/\\",
            BinaryOpType::Or => "\\/",
            BinaryOpType::Xor => "^",
            BinaryOpType::Div => "/",
            BinaryOpType::Eq => "=",
            BinaryOpType::Exp => "**",
        })
    }

    fn visit_block(&mut self, block: &Block) -> Result<()> {
        self.println("{")?;
        self.visit_statements(&block.statements)?;
        self.println("}")
    }

    fn visit_definition(&mut self, definition: &Definition) -> Result<()> {
        self.print(&definition.identifier)
    }

    fn visit_expression(&mut self, expression: &Expression) -> Result<()> {
        match expression {
            Expression::Unary(op, expr) => {
                self.print("(")?;
                self.visit_unary_op_type(op)?;
                self.print(" ")?;
                self.visit_expression(expr)?;
                self.print(")")
            }
            Expression::Binary(op, lhs, rhs) => {
                self.print("(")?;
                self.visit_expression(lhs)?;
                self.print(" ")?;
                self.visit_binary_op_type(op)?;
                self.print(" ")?;
                self.visit_expression(rhs)?;
                self.print(")")
            }
            Expression::ECall(ecall) => self.visit_function_call(ecall),
            Expression::Literal(literal) => self.visit_literal(literal),
            Expression::Reference(reference) => self.visit_reference(reference),
            Expression::Tuple(expressions) => {
                self.print("(")?;
                for (i, expr) in expressions.iter().enumerate() {
                    if i > 0 {
                        self.print(", ")?;
                    }
                    self.visit_expression(expr)?;
                }
                self.print(")")
            }
        }
    }

    fn visit_function(&mut self, function: &Function) -> Result<()> {
        self.print(format!("op {}", function.name).as_str())?;
        self.visit_signature(&function.signature)?;
        self.print(" = ")?;
        self.visit_expression(&function.body)?;
        self.println(".")
    }

    fn visit_function_call(&mut self, call: &FunctionCall) -> Result<()> {
        let FunctionCall { target, arguments } = call;
        if !arguments.is_empty() {
            self.print("(")?;
            self.visit_reference(target)?;
            for arg in arguments {
                self.print(" ")?;
                self.visit_expression(arg)?;
            }
            self.print(")")
        } else {
            self.visit_reference(target)
        }
    }

    fn visit_integer_literal(&mut self, int_literal: &IntegerLiteral) -> Result<()> {
        self.print(&int_literal.inner)
    }

    fn visit_literal(&mut self, literal: &Literal) -> Result<()> {
        match literal {
            Literal::StringPlaceholder(s) => {
                self.print("STRING (*")?;
                self.print(s.as_str())?;
                self.print("*)")
            }
            Literal::Int(int_literal) => self.visit_integer_literal(int_literal),
            Literal::Bool(value) => self.print(format!("{value}").as_str()),
        }
    }
    fn visit_module(&mut self, module: &Module) -> Result<()> {
        const NAME_ANONYMOUS_MODULE: &str = "ANONYMOUS";
        let module_name = module
            .name
            .clone()
            .unwrap_or(String::from(NAME_ANONYMOUS_MODULE));

        self.println(format!("(* Begin {} *)", module_name).as_str())?;

        let names = module.names_ordered();
        //FIXME: dependency order
        for name in &names {
            let def = module.definitions.get(name).unwrap();
            if def.is_fun_def() {
                self.visit_module_definition(def)?;
                self.println("")?;
            }
        }

        self.print("module ")?;
        self.print(&module_name)?;
        self.println(" = {")?;
        self.increase_indent()?;

        for name in &names {
            let def = module.definitions.get(name).unwrap();
            if def.is_proc_def() {
                self.visit_module_definition(def)?;
                self.println("")?;
            }
        }

        self.println("")?;
        self.println("}.")?;
        self.decrease_indent()?;
        self.println(format!("(* End {} *)", module_name).as_str())
    }

    fn visit_module_definition(&mut self, definition: &TopDefinition) -> Result<()> {
        match definition {
            TopDefinition::Proc(proc_def) => self.visit_proc(proc_def),
            TopDefinition::Function(fun_def) => self.visit_function(fun_def),
        }
    }

    fn visit_proc_call(&mut self, call: &ProcCall) -> Result<()> {
        let ProcCall { target, arguments } = call;
        self.visit_reference(target)?;
        self.print("(")?;
        for (i, arg) in arguments.iter().enumerate() {
            if i > 0 {
                self.print(", ")?;
            }
            self.visit_expression(arg)?;
        }
        self.print(")")
    }

    fn visit_proc(&mut self, proc: &Proc) -> Result<()> {
        // FIXME temp
        // if proc.name.name == IMPLICIT_CODE_FUNCTION_NAME {
        //     return;
        // }
        self.print("proc ")?;
        self.print(&proc.name)?;
        self.visit_signature(&proc.signature)?;
        self.println(" = {")?;
        self.visit_locals(&proc.locals)?;
        self.visit_statements(&proc.body.statements)?;
        self.println("}")
    }

    fn visit_reference(&mut self, reference: &Reference) -> Result<()> {
        for module in &reference.path.stack {
            self.print(&format!("{}", module))?;
            self.print(".")?;
        }
        self.print(&reference.identifier)
    }

    fn visit_signature(&mut self, signature: &Signature) -> Result<()> {
        let Signature {
            formal_parameters,
            return_type,
            kind,
        } = signature;
        if kind != &SignatureKind::Function || !formal_parameters.is_empty() {
            self.print("(")?;

            for (i, def) in formal_parameters.iter().enumerate() {
                if i > 0 {
                    self.print(", ")?
                }
                self.print(format!("{} : {}", def.identifier, def.get_effective_type(),).as_str())?;
            }
            self.print(")")?;
        }
        self.print(format!(": {}", return_type).as_str())
    }

    fn visit_statement(&mut self, statement: &Statement) -> Result<()> {
        fn print_lhs_references<T>(s: &mut T, references: &[Reference]) -> Result<()>
        where
            T: IPrinter + Visitor,
        {
            match references.len() {
                0 => Ok(()),
                1 => s.visit_reference(&references[0]),
                _ => {
                    s.print("(")?;
                    for (i, r) in references.iter().enumerate() {
                        if i > 0 {
                            s.print(",")?;
                        }
                        s.visit_reference(r)?;
                    }
                    s.print(")")
                }
            }
        }

        match statement {
            Statement::Expression(expression) => self.visit_expression(expression),
            Statement::Block(block) => self.visit_block(block),
            Statement::IfConditional(if_conditional) => self.visit_if_conditional(if_conditional),
            Statement::EAssignment(refs, rhs) => {
                print_lhs_references(self, refs)?;
                if !refs.is_empty() {
                    self.print(" <- ")?;
                }
                self.visit_expression(rhs)
            }
            Statement::PAssignment(refs, rhs) => {
                print_lhs_references(self, refs)?;
                if !refs.is_empty() {
                    self.print(" <@ ")?;
                }
                self.visit_proc_call(rhs)
            }
            Statement::Return(e) => {
                self.print("return ")?;
                self.visit_expression(e)
            }
            Statement::WhileLoop(while_loop) => self.visit_while_loop(while_loop),
        }
    }

    fn visit_type(&mut self, r#type: &Type) -> Result<()> {
        self.print(format!("{}", r#type).as_str())
    }

    fn visit_unary_op_type(&mut self, op: &UnaryOpType) -> Result<()> {
        self.print(match op {
            UnaryOpType::Neg => "-",
            UnaryOpType::Not => "!",
        })
    }

    fn visit_if_conditional(&mut self, conditional: &IfConditional) -> Result<()> {
        let IfConditional { condition, yes, no } = conditional;

        self.print("if (")?;
        self.visit_expression(condition)?;
        self.println(")")?;

        if !yes.is_block() {
            self.print(" { ")?;
        }
        self.visit_statement(yes)?;
        if !yes.is_block() {
            self.println(" } ")?;
        }
        if let Some(no) = no {
            self.println("")?;
            self.print("else ")?;
            self.visit_statement(no)?;
        }
        Ok(())
    }

    fn visit_while_loop(
        &mut self,
        while_loop: &super::syntax::statement::while_loop::WhileLoop,
    ) -> Result<()> {
        let WhileLoop { condition, body } = while_loop;
        self.print("while (")?;

        self.visit_expression(condition)?;
        self.println(")")?;

        if !body.is_block() {
            self.print(" { ")?;
        }
        self.visit_statement(body)?;
        if !body.is_block() {
            self.println(" } ")?;
        }
        Ok(())
    }
}

impl<W> ECPrinter<W>
where
    W: std::fmt::Write,
{
    fn visit_statements(&mut self, block: &Vec<Statement>) -> Result<()> {
        self.increase_indent()?;
        for statement in block {
            self.visit_statement(statement)?;
            if statement_followed_by_semicolon(statement) {
                self.print(";")?;
            }
            self.println("")?;
        }
        self.println("")?;
        self.decrease_indent()
    }
}
