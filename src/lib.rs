mod opts;

pub use opts::Options;

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

#[derive(Debug, Clone, PartialEq)]
pub struct VarContext {
    pub name: String,
    pub var_type: VarContextType,
    pub is_member: bool,
    pub is_const: bool,
    pub is_static: bool,
    pub src_location: SrcLocation,
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
        VarContext {
            name: entity.get_name().unwrap(),
            var_type,
            is_member: entity.get_kind() == clang::EntityKind::FieldDecl
                || parent.get_kind() == clang::EntityKind::StructDecl
                || parent.get_kind() == clang::EntityKind::ClassDecl,
            is_const,
            is_static: entity.get_linkage().unwrap() != clang::Linkage::Automatic,
            src_location: SrcLocation::from(entity),
        }
    }
}

pub fn parse_file<F: FnMut(VarContext)>(options: Options, mut callback: F) {
    log::debug!("Using {}", clang::get_version());
    let c = clang::Clang::new().expect("Failed to create basic clang object");
    let i = clang::Index::new(&c, false, options.verbose > 0);
    let mut cpp_arguments = vec!["-x", "c++", "-std=c++11"];
    for i in options.includes.iter() {
        cpp_arguments.push("-I");
        cpp_arguments.push(i.as_str());
    }
    let mut p = i.parser(&options.input);
    if options.verbose == 2 {
        log::debug!("Parsing with arguments: {:?}", cpp_arguments);
    }
    p.arguments(&cpp_arguments[..]);
    let tu = p.parse();
    if let Err(e) = tu {
        log::error!("Failed to parse file with error {}", e);
        return ();
    }

    let tu = tu.unwrap();
    let entity = tu.get_entity();
    log::debug!("Parsing translation unit: {:?}", &entity);
    if let Some(l) = entity.get_language() {
        log::debug!("language for TU is {:?}", l);
    }

    entity.visit_children(|entity, parent| {
        let loc = entity.get_location();
        if let Some(l) = loc {
            if !l.is_in_main_file() {
                return clang::EntityVisitResult::Recurse;
            }
        }

        if options.debug {
            log::debug!("Entity item: {:?}", &entity);
        }

        match entity.get_kind() {
            clang::EntityKind::VarDecl | clang::EntityKind::FieldDecl => {
                // log::debug!("Parsing {:?}", entity.get_type().unwrap().get_kind());
                callback(VarContext::from(&entity, &parent));
            }
            _ => (),
        }
        return clang::EntityVisitResult::Recurse;
    });
}
