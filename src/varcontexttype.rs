#[derive(Debug, PartialEq, Clone)]
pub enum VarContextType {
    Value,
    Ptr,
    Ref,
}

impl VarContextType {
    pub fn from(entity: &clang::Entity) -> Self {
        match entity.get_type().unwrap().get_kind() {
            clang::TypeKind::Pointer => VarContextType::Ptr,
            clang::TypeKind::LValueReference => VarContextType::Ref,
            clang::TypeKind::RValueReference => VarContextType::Ref,
            _ => VarContextType::Value,
        }
    }
}
