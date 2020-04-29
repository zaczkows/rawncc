#[derive(Debug, Clone, PartialEq)]
pub struct SrcLocation {
    pub file: String,
    pub line_no: u32,
    pub column: u32,
}

impl SrcLocation {
    pub fn from(entity: &clang::Entity) -> Self {
        let loc = entity.get_location().unwrap().get_file_location();
        SrcLocation {
            file: String::from(loc.file.unwrap().get_path().to_str().unwrap()),
            line_no: loc.line,
            column: loc.column,
        }
    }
}
