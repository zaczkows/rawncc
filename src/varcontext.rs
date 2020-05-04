use crate::srclocation::SrcLocation;
use crate::varcontexttype::VarContextType;

#[derive(Debug, Clone, PartialEq)]
pub struct VarContext {
    pub name: String,
    pub var_type: VarContextType,
    pub is_member: bool,
    pub is_const: bool,
    pub is_static: bool,
    pub src_location: SrcLocation,
}

fn is_member_variable(entity: &clang::Entity, parent: &clang::Entity) -> bool {
    // This is needed in case of class static variable initialization
    // i.e. const int CLASS_NAME::VARIABLE = 42;
    let semantic_parent = entity.get_semantic_parent();
    let is_semantic_parent_a_class = if semantic_parent.is_some() {
        let kind = semantic_parent.unwrap().get_kind();
        kind == clang::EntityKind::StructDecl || kind == clang::EntityKind::ClassDecl
    } else {
        false
    };

    entity.get_kind() == clang::EntityKind::FieldDecl
        || parent.get_kind() == clang::EntityKind::StructDecl
        || parent.get_kind() == clang::EntityKind::ClassDecl
        || is_semantic_parent_a_class
}

impl VarContext {
    pub fn from(entity: &clang::Entity, parent: &clang::Entity) -> Self {
        assert!(
            entity.get_kind() == clang::EntityKind::VarDecl
                || entity.get_kind() == clang::EntityKind::FieldDecl
        );
        let var_type = VarContextType::from(entity);
        let context_type = entity.get_type().unwrap();
        let is_const = (if var_type != VarContextType::Value {
            context_type.get_pointee_type().unwrap()
        } else {
            context_type
        })
        .is_const_qualified();
        let name = entity.get_name().unwrap();
        let is_static = entity.get_linkage().unwrap() != clang::Linkage::Automatic;
        VarContext {
            name,
            var_type,
            is_member: is_member_variable(entity, parent),
            is_const,
            is_static,
            src_location: SrcLocation::from(entity),
        }
    }
}
