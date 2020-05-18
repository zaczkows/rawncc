use crate::srclocation::SrcLocation;

#[derive(Debug, Clone, PartialEq)]
pub struct CastContext {
    pub location: SrcLocation,
}
