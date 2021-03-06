use super::*;

mod function_visitor;
pub use function_visitor::FunctionVisitor;

mod generic_call_visitor;
pub use generic_call_visitor::GenericCallsVisitor;

mod context_building_visitor;
pub use context_building_visitor::ContextBuildingVisitor;

mod library_emitter_visitor;
pub use library_emitter_visitor::LibraryEmitterVisitor;

pub mod implementations;

pub trait Visitor {
  fn visit_function_declaration(&mut self, _: &FunctionDeclaration) {}
  fn visit_class_declaration(&mut self, _: &ClassDeclaration) {}
  fn visit_generic_function_call(&mut self, _: &FunctionCall) {}
  fn visit_generic_variable_declaration(&mut self, _: &TypeDeclaration) {}
  fn visitor_type(&self) -> VisitorType;
}

pub trait Visited {
  fn accept<T: Visitor>(&self, visitor: &mut T);
}

pub enum VisitorType {
  FunctionDeclarationVisitor,
  GenericCallsVisitor,
  ContextBuildingVisitor,
  LibraryEmitterVisitor,
}
