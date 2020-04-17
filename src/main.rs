use rawncc;

#[derive(Debug)]
enum VarContextType {
    Value,
    Ptr,
    Ref,
}

impl VarContextType {
    fn from(entity: &clang::Entity) -> Self {
        match entity.get_type().unwrap().get_kind() {
            clang::TypeKind::Pointer => VarContextType::Ptr,
            clang::TypeKind::LValueReference => VarContextType::Ref,
            clang::TypeKind::RValueReference => VarContextType::Ref,
            _ => VarContextType::Value,
        }
    }
}

#[derive(Debug)]
struct SrcLocation {
    file: String,
    line_no: u32,
    column: u32,
    offset: u32,
}

impl SrcLocation {
    fn from(entity: &clang::Entity) -> Self {
        let loc = entity.get_location().unwrap().get_file_location();
        SrcLocation {
            file: String::from(loc.file.unwrap().get_path().to_str().unwrap()),
            line_no: loc.line,
            column: loc.column,
            offset: loc.offset,
        }
    }
}

#[derive(Debug)]
struct VarContext {
    name: String,
    var_type: VarContextType,
    is_member: bool,
    is_const: bool,
    src_location: SrcLocation,
}

impl VarContext {
    fn from(entity: &clang::Entity) -> Self {
        assert!(
            entity.get_kind() == clang::EntityKind::VarDecl
                || entity.get_kind() == clang::EntityKind::FieldDecl
        );
        VarContext {
            name: entity.get_name().unwrap(),
            var_type: VarContextType::from(entity),
            is_member: entity.get_kind() == clang::EntityKind::FieldDecl,
            is_const: entity.get_type().unwrap().is_const_qualified(),
            src_location: SrcLocation::from(entity),
        }
    }
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    let options = rawncc::parse_cmd_line_args();

    log::debug!("Using {}", clang::get_version());
    let c = clang::Clang::new().expect("Failed to create basic clang object");
    let i = clang::Index::new(&c, false, options.verbose > 0);
    let mut p = i.parser(&options.input);
    p.arguments(&["-x", "c++", "-std=c++11"]);
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

    let var_handler = |context| {
        log::debug!("Found variable: {:?}", context);
    };

    entity.visit_children(|entity, _parent| {
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
                log::debug!("Parsing {:?}", entity.get_type().unwrap().get_kind());
                var_handler(VarContext::from(&entity));
            }
            _ => (),
        }
        return clang::EntityVisitResult::Recurse;
    });
}
