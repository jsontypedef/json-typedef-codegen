use super::ast;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct Ir {
    pub version: String,
    pub imports: BTreeMap<String, BTreeSet<String>>,
    pub classes: BTreeMap<String, Class>,
}

impl Ir {
    pub fn from_ast(ast: ast::Ast) -> Self {
        let lookup_table: Vec<_> = ast.classes.into_lookup_table().collect();
        let lookup_state = LookupState {
            lookup_table: lookup_table.iter().map(|e| e.0.clone()).collect(),
            definition_names: ast.definition_names,
        };

        Self {
            version: ast.version,
            imports: ast.imports,
            classes: lookup_table
                .into_iter()
                .map(|(k, v)| (k, Class::from_ast(&lookup_state, v)))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub enum Class {
    TypeWrapper(TypeWrapper),
    Enum(Enum),
    Dataclass(Dataclass),
    Discriminator(Discriminator),
}

impl Class {
    fn from_ast(state: &LookupState, class: ast::Class) -> Self {
        match class {
            ast::Class::TypeWrapper(t) => Class::TypeWrapper(TypeWrapper::from_ast(state, t)),
            ast::Class::Enum(t) => Class::Enum(Enum::from_ast(state, t)),
            ast::Class::Dataclass(t) => Class::Dataclass(Dataclass::from_ast(state, t)),
            ast::Class::Discriminator(t) => Class::Discriminator(Discriminator::from_ast(state, t)),
        }
    }
}

#[derive(Debug)]
pub struct TypeWrapper {
    pub description: String,
    pub type_: String,
}

impl TypeWrapper {
    fn from_ast(state: &LookupState, ast: ast::TypeWrapper) -> Self {
        Self {
            description: ast.description,
            type_: state.render_typeref(ast.type_),
        }
    }
}

#[derive(Debug)]
pub struct Enum {
    pub description: String,
    pub members: BTreeMap<String, EnumMember>,
}

impl Enum {
    fn from_ast(_state: &LookupState, ast: ast::Enum) -> Self {
        Self {
            description: ast.description,
            members: ast
                .members
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        EnumMember {
                            description: v.description,
                            value: v.value,
                        },
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct EnumMember {
    pub description: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Dataclass {
    pub description: String,
    pub fields: BTreeMap<String, DataclassField>,
}

impl Dataclass {
    fn from_ast(state: &LookupState, ast: ast::Dataclass) -> Self {
        Self {
            description: ast.description,
            fields: ast
                .fields
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DataclassField {
                            description: v.description,
                            json_name: v.json_name,
                            type_: state.render_typeref(v.type_),
                        },
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct DataclassField {
    pub description: String,
    pub json_name: String,
    pub type_: String,
}

#[derive(Debug)]
pub struct Discriminator {
    pub description: String,
    pub discriminator_name: String,
    pub discriminator_json_name: String,
    pub variants: BTreeMap<String, DiscriminatorVariant>,
}

impl Discriminator {
    fn from_ast(state: &LookupState, ast: ast::Discriminator) -> Self {
        Self {
            description: ast.description,
            discriminator_name: ast.discriminator_name,
            discriminator_json_name: ast.discriminator_json_name,
            variants: ast
                .variants
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DiscriminatorVariant {
                            discriminator_value: v.discriminator_value,
                            type_: state.render_typeref(v.type_),
                        },
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct DiscriminatorVariant {
    pub discriminator_value: String,
    pub type_: String,
}

struct LookupState {
    lookup_table: Vec<String>,
    definition_names: BTreeMap<String, usize>,
}

impl LookupState {
    fn render_typeref(&self, t: ast::TypeRef) -> String {
        match t {
            ast::TypeRef::Primitive(s) => s,
            ast::TypeRef::Identifier(id) => self.lookup_table[id].clone(),
            ast::TypeRef::Definition(def) => {
                self.render_typeref(ast::TypeRef::Identifier(self.definition_names[&def]))
            }
            ast::TypeRef::Optional(t) => format!("Optional[{}]", self.render_typeref(*t)),
            ast::TypeRef::ListOf(t) => format!("List[{}]", self.render_typeref(*t)),
            ast::TypeRef::DictOf(t) => format!("Dict[str, {}]", self.render_typeref(*t)),
        }
    }
}
