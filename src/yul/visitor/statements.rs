//!
//! Iterate over all statements in YUL syntax tree using depth-first search in
//! post-order traversal.
//!

use std::marker::PhantomData;

use crate::yul::path::builder::Builder;
use crate::yul::path::tracker::PathTracker;
use crate::yul::path::Path;
use era_yul::yul::parser::dialect::Dialect;
use era_yul::yul::parser::statement::block::Block;
use era_yul::yul::parser::statement::code::Code;
use era_yul::yul::parser::statement::for_loop::ForLoop;
use era_yul::yul::parser::statement::function_definition::FunctionDefinition;
use era_yul::yul::parser::statement::if_conditional::IfConditional;
use era_yul::yul::parser::statement::object::Object;
use era_yul::yul::parser::statement::switch::Switch;
use era_yul::yul::parser::statement::Statement;
use era_yul::yul::visitor::Visitor;

///
/// State of statement visitor.
///
pub trait StatementAction<P>
where
    P: Dialect,
{
    /// Action to perform on each statement.
    fn action(&mut self, statement: &Statement<P>, path: &Path);
}

///
/// Iterate over all statements in YUL syntax tree using depth-first search in
/// post-order traversal.
///
pub struct Statements<A, P>
where
    A: StatementAction<P>,
    P: Dialect,
{
    /// Action to perform on each statement.
    pub action: A,
    tracker: Builder,
    _marker: PhantomData<P>,
}

impl<A, P> Statements<A, P>
where
    A: StatementAction<P>,
    P: Dialect,
{
    ///
    /// Returns a new instance of [`Statements`].
    ///
    pub fn new(action: A, path: Path) -> Self {
        Self {
            action,
            tracker: Builder::new(path),
            _marker: PhantomData,
        }
    }
}

impl<A, P> Visitor<P> for Statements<A, P>
where
    A: StatementAction<P>,
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
            Statement::FunctionDefinition(fd) => self.visit_function_definition(fd),
            Statement::VariableDeclaration(_) | Statement::Assignment(_) => (),
            Statement::IfConditional(if_conditional) => self.visit_if_conditional(if_conditional),
            Statement::Switch(switch) => self.visit_switch(switch),
            Statement::ForLoop(for_loop) => self.visit_for_loop(for_loop),
            Statement::Continue(_) | Statement::Break(_) | Statement::Leave(_) => (),
        };
        self.action.action(stmt, self.tracker.here())
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
        self.tracker.leave();
    }
}
