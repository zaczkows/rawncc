use crate::srclocation::SrcLocation;

#[derive(Debug, Clone, PartialEq)]
pub enum FnType {
    Function,
    Method,
    Ctor,
    Dtor,
}

pub(crate) fn is_fn_type(kind: &clang::EntityKind) -> Option<FnType> {
    match kind {
        clang::EntityKind::FunctionDecl => Some(FnType::Function),
        clang::EntityKind::Method => Some(FnType::Method),
        clang::EntityKind::Constructor => Some(FnType::Ctor),
        clang::EntityKind::Destructor => Some(FnType::Dtor),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnContext {
    pub name: String,
    pub fn_type: FnType,
    pub location: SrcLocation,
}

impl FnContext {
    pub(crate) fn from(entity: &clang::Entity) -> Self {
        assert!(is_fn_type(&entity.get_kind()).is_some());

        FnContext {
            name: entity.get_name().unwrap(),
            fn_type: is_fn_type(&entity.get_kind()).unwrap(),
            location: SrcLocation::from(&entity),
        }
    }
}
