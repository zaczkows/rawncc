use crate::srclocation::SrcLocation;

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexType {
    Class,
    Enum,
    Struct,
    Union,
}

pub(crate) fn get_complex_type(kind: &clang::EntityKind) -> Option<ComplexType> {
    match kind {
        clang::EntityKind::StructDecl => Some(ComplexType::Struct),
        clang::EntityKind::ClassDecl => Some(ComplexType::Class),
        clang::EntityKind::EnumDecl => Some(ComplexType::Enum),
        clang::EntityKind::UnionDecl => Some(ComplexType::Union),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComplexContext {
    pub name: String,
    pub c_type: ComplexType,
    pub location: SrcLocation,
}

impl ComplexContext {
    pub(crate) fn from(entity: &clang::Entity) -> Self {
        assert!(get_complex_type(&entity.get_kind()).is_some());

        ComplexContext {
            name: entity.get_name().unwrap_or(String::from("")),
            c_type: get_complex_type(&entity.get_kind()).unwrap(),
            location: SrcLocation::from(&entity),
        }
    }
}
