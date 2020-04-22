use rawncc;

#[derive(Debug, PartialEq)]
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
}

impl SrcLocation {
    fn from(entity: &clang::Entity) -> Self {
        let loc = entity.get_location().unwrap().get_file_location();
        SrcLocation {
            file: String::from(loc.file.unwrap().get_path().to_str().unwrap()),
            line_no: loc.line,
            column: loc.column,
        }
    }
}

#[derive(Debug)]
struct VarContext {
    name: String,
    var_type: VarContextType,
    is_member: bool,
    is_const: bool,
    is_static: bool,
    src_location: SrcLocation,
}

impl VarContext {
    fn from(entity: &clang::Entity) -> Self {
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
            is_member: entity.get_accessibility().is_some(),
            is_const,
            is_static: entity.get_linkage().unwrap() != clang::Linkage::Automatic,
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

    let var_handler = |context: VarContext| {
        log::debug!("Found variable: {:?}", context);
        let mut regex_str = String::new();
        let static_const = context.is_static && context.is_const;
        if static_const {
            if context.is_member {
                regex_str += "([A-Z]+_)+[A-Z]+";
            } else {
                regex_str += "m_";
            }
        } else {
            if context.var_type == VarContextType::Ptr {
                regex_str += "p";
            }
            if context.var_type == VarContextType::Ref {
                regex_str += "r";
            }

            regex_str += "([A-Z][a-z0-9]+)+";
        }

        let r = regex::Regex::new(regex_str.as_str()).unwrap();
        if !r.is_match(context.name.as_str()) {
            log::debug!("Invalid naming");
        }
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
                // log::debug!("Parsing {:?}", entity.get_type().unwrap().get_kind());
                var_handler(VarContext::from(&entity));
            }
            _ => (),
        }
        return clang::EntityVisitResult::Recurse;
    });
}
