use crate::srclocation::SrcLocation;

#[derive(Debug, PartialEq, Clone)]
pub enum VarContextType {
    Value,
    Ptr,
    Ref,
    Array,
}

impl VarContextType {
    pub fn from(entity: &clang::Entity) -> Self {
        let kind = entity.get_type().unwrap().get_kind();
        match kind {
            clang::TypeKind::Pointer
            | clang::TypeKind::BlockPointer
            | clang::TypeKind::MemberPointer => VarContextType::Ptr,
            clang::TypeKind::LValueReference | clang::TypeKind::RValueReference => {
                VarContextType::Ref
            }
            clang::TypeKind::ConstantArray
            | clang::TypeKind::DependentSizedArray
            | clang::TypeKind::IncompleteArray
            | clang::TypeKind::VariableArray => VarContextType::Array,
            _ => {
                // log::debug!("Found unhandled {:?} kind", &kind);
                VarContextType::Value
            }
        }
    }
}

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

fn is_const_type(entity: &clang::Entity, var_type: &VarContextType) -> bool {
    let context_type = entity.get_type().unwrap();
    let is_const = match *var_type {
        VarContextType::Value => context_type.is_const_qualified(),
        VarContextType::Ptr | VarContextType::Ref => context_type
            .get_pointee_type()
            .unwrap()
            .is_const_qualified(),
        VarContextType::Array => {
            entity.get_type().unwrap().get_kind() == clang::TypeKind::ConstantArray
        }
    };

    is_const
}

impl VarContext {
    pub fn from(entity: &clang::Entity, parent: &clang::Entity) -> Self {
        assert!(
            entity.get_kind() == clang::EntityKind::VarDecl
                || entity.get_kind() == clang::EntityKind::FieldDecl
        );
        let var_type = VarContextType::from(entity);
        let name = entity.get_name().unwrap();
        let linkage = entity.get_linkage().unwrap();
        let is_const = is_const_type(entity, &var_type);
        VarContext {
            name,
            var_type,
            is_member: is_member_variable(entity, parent),
            is_const,
            is_static: linkage != clang::Linkage::Automatic,
            src_location: SrcLocation::from(entity),
        }
    }
}
