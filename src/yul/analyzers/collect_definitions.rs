//!
//! Collect all the definitions in YUL program and derive their EasyCrypt types.
//!

use std::collections::HashMap;

use crate::yul::path::builder::Builder;
use crate::yul::path::full_name::FullName;
use crate::yul::path::tracker::PathTracker as _;
use crate::yul::path::Path;
use crate::YulVisitor;
use era_yul::yul::parser::dialect::Dialect;
use era_yul::yul::parser::identifier::Identifier;
use era_yul::yul::parser::r#type::Type as YulType;
use era_yul::yul::parser::statement::block::Block;
use era_yul::yul::parser::statement::code::Code;
use era_yul::yul::parser::statement::for_loop::ForLoop;
use era_yul::yul::parser::statement::function_definition::FunctionDefinition;
use era_yul::yul::parser::statement::if_conditional::IfConditional;
use era_yul::yul::parser::statement::object::Object;
use era_yul::yul::parser::statement::switch::Switch;
use era_yul::yul::parser::statement::variable_declaration::VariableDeclaration;
use era_yul::yul::parser::statement::Statement;
use era_yul::yul::visitor::implicit_code_function;

use super::Definition;
use super::Function;
use super::Variable;

///
/// Collect all definitions in the YUL code.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectDefinitions<P>
where
    P: Dialect,
{
    pub result: HashMap<FullName, Definition<P>>,
    pub tracker: Builder,
}

impl<P> Default for CollectDefinitions<P>
where
    P: Dialect,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P> CollectDefinitions<P>
where
    P: Dialect,
{
    pub fn new() -> Self {
        Self {
            result: HashMap::new(),
            tracker: Builder::new(Path::empty()),
        }
    }

    fn derive_type(typ: &Option<YulType>) -> super::Type {
        match typ {
            Some(typ) => super::Type::Atomic(typ.clone()),
            None => super::Type::default(),
        }
    }

    fn add_var(&mut self, binding: &Identifier, path: &Path) {
        let name = &binding.inner;
        let full_name = FullName::custom(name, path.clone());
        let definition = Definition::Variable(Variable {
            name: full_name.clone(),
            typ: Self::derive_type(&binding.r#type),
        });
        let _ = self.result.insert(full_name, definition);
    }

    fn add_variable_declaration(
        &mut self,
        variable_declaration: &VariableDeclaration,
        path: &Path,
    ) {
        for binding in &variable_declaration.bindings {
            self.add_var(binding, path);
        }
    }

    fn add_function_definition(
        &mut self,
        function_definition: &FunctionDefinition<P>,
        path: &Path,
    ) {
        let FunctionDefinition {
            identifier,
            arguments,
            result,
            ..
        } = function_definition;

        for var in arguments.iter().chain(result.iter()) {
            self.add_var(var, path)
        }

        let name = &identifier.to_string();

        let full_name = FullName::custom(name, path.clone());

        let definition = {
            let return_type: Vec<_> = result
                .iter()
                .map(|r| Self::derive_type(&r.r#type))
                .collect();
            let arguments_type: Vec<_> = arguments
                .iter()
                .map(|arg| Self::derive_type(&arg.r#type))
                .collect();
            let function_type = super::Type::Function {
                arguments_type,
                return_type,
            };
            Definition::Function(Function {
                full_name: full_name.clone(),
                body: function_definition.clone(),
                typ: function_type,
            })
        };

        self.result.insert(full_name, definition);
    }
}

impl<P> YulVisitor<P> for CollectDefinitions<P>
where
    P: Dialect,
{
    fn visit_switch(&mut self, switch: &Switch<P>) {
        let Switch { cases, default, .. } = switch;
        for case in cases {
            self.visit_block(&case.block)
        }
        if let Some(block) = default {
            self.visit_block(block)
        }
    }

    fn visit_object(&mut self, object: &Object<P>) {
        self.tracker.enter_object(&object.identifier);
        self.visit_code(&object.code);

        if let Some(inner_object) = &object.inner_object {
            self.visit_object(inner_object);
        }

        self.tracker.leave()
    }

    fn visit_for_loop(&mut self, for_loop: &ForLoop<P>) {
        self.tracker.enter_for1();
        self.visit_block(&for_loop.initializer);
        self.tracker.leave();
        self.tracker.enter_for2();
        self.visit_block(&for_loop.finalizer);
        self.tracker.leave();
        self.tracker.enter_for3();
        self.visit_block(&for_loop.body);
        self.tracker.leave();
    }

    fn visit_function_definition(&mut self, function_definition: &FunctionDefinition<P>) {
        let FunctionDefinition {
            identifier, body, ..
        } = function_definition;
        self.tracker.enter_function(identifier);
        self.visit_block(body);
        self.tracker.leave();
    }

    fn visit_if_conditional(&mut self, if_conditional: &IfConditional<P>) {
        self.tracker.enter_if_then();
        self.visit_block(&if_conditional.block);
        self.tracker.leave();
    }

    fn visit_statement(&mut self, stmt: &Statement<P>) {
        match stmt {
            Statement::Object(object) => self.visit_object(object),
            Statement::Code(code) => self.visit_code(code),
            Statement::Block(block) => self.visit_block(block),
            Statement::Expression(_) => (),
            Statement::FunctionDefinition(fd) => {
                self.visit_function_definition(fd);
                self.add_function_definition(fd, &self.tracker.here().clone())
            }
            Statement::VariableDeclaration(vd) => {
                self.add_variable_declaration(vd, &self.tracker.here().clone())
            }
            Statement::Assignment(_) => (),
            Statement::IfConditional(if_conditional) => self.visit_if_conditional(if_conditional),
            Statement::Switch(switch) => self.visit_switch(switch),
            Statement::ForLoop(for_loop) => self.visit_for_loop(for_loop),
            Statement::Continue(_) | Statement::Break(_) | Statement::Leave(_) => (),
        };
    }

    fn visit_block(&mut self, block: &Block<P>) {
        self.tracker.enter_block();

        for statement in &block.statements {
            self.visit_statement(statement)
        }
        self.tracker.leave();
    }

    fn visit_code(&mut self, code: &Code<P>) {
        self.tracker.enter_code();
        self.visit_block(&code.block);
        self.add_function_definition(&implicit_code_function(code), &self.tracker.here().clone());
        self.tracker.leave();
    }
}

#[cfg(test)]
mod test {
    use era_yul::yul::{
        lexer::Lexer,
        parser::{dialect::DefaultDialect, statement::object::Object},
        visitor::Visitor,
    };

    use super::CollectDefinitions;

    #[test]
    fn collect_definitions() {
        let input = r#"

object "ecadd" {
    code { }
    object "ecadd_deployed" {
        code {

            {
                let x := 0
                    switch calldataload(4)
                    case 0 {
                        x := calldataload(0x24)
                    }
                default {
                    x := calldataload(0x44)
                }
                sstore(0, div(x, 2))
            }
let y := 10
        }
    }

}
"#
        .to_string();
        let mut lexer = Lexer::new(input);
        let object = Object::<DefaultDialect>::parse(&mut lexer, None).unwrap();
        let mut collector = CollectDefinitions::new();
        collector.visit_object(&object);
        let mut expected = vec!["x", "y", "BODY", "BODY"];
        expected.sort();
        let mut produced = collector
            .result
            .keys()
            .map(|fullname| era_yul::yul::printer::name_identifier(&fullname.name))
            .collect::<Vec<_>>();
        produced.sort();
        assert_eq!(expected, produced);
    }
}
