use rawncc;

#[derive(Debug)]
enum VarContextType {
    None,
    Ptr,
    Ref,
}

impl VarContextType {
    fn from(entity: &clang::Entity) -> Self {
        match entity.get_type().unwrap().get_kind() {
            clang::TypeKind::Pointer => VarContextType::Ptr,
            clang::TypeKind::LValueReference => VarContextType::Ref,
            clang::TypeKind::RValueReference => VarContextType::Ref,
            _ => VarContextType::None,
        }
    }
}

#[derive(Debug)]
struct VarContext {
    name: String,
    vtype: VarContextType,
}

impl VarContext {
    fn from(entity: &clang::Entity) -> Self {
        assert!(entity.get_kind() == clang::EntityKind::VarDecl);
        VarContext {
            name: entity.get_name().unwrap(),
            vtype: VarContextType::from(entity),
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
    let i = clang::Index::new(&c, false, options.debug);
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

    let file_location = options.input.to_str().expect("Invalid filename");
    entity.visit_children(|entity, _parent| {
        let loc = entity.get_location();
        if let Some(l) = loc {
            if let Some(f) = l.get_file_location().file {
                if f.get_path().to_str().unwrap() != file_location {
                    return clang::EntityVisitResult::Recurse;
                }
            }
        }

        if entity.get_kind() == clang::EntityKind::VarDecl {
            let vc = VarContext::from(&entity);
            log::debug!("Parsing {:?}", entity.get_type().unwrap().get_kind());
            log::debug!("Found variable: {:?}", vc);
        }
        return clang::EntityVisitResult::Recurse;
    });
}
