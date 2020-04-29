mod opts;
mod srclocation;
mod varcontext;
mod varcontexttype;

pub use opts::Options;
pub use srclocation::SrcLocation;
pub use varcontext::VarContext;
pub use varcontexttype::VarContextType;

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

pub fn check_ra_nc(context: &VarContext) -> Result<(), String> {
    let mut regex_str = String::from("^");
    let static_const = context.is_static && context.is_const;
    if static_const {
        regex_str += "([A-Z]+_)+[A-Z]+";
    } else {
        if context.is_member {
            regex_str += "m_";
        }

        let uppercase_first = "([A-Z][a-z0-9]+)+";
        let lowercase_first = "[a-z0-9]+([A-Z][a-z0-9]+)*";
        if context.var_type == VarContextType::Ptr {
            regex_str += "p";
            regex_str += uppercase_first;
        }
        else if context.var_type == VarContextType::Ref {
            regex_str += "r";
            regex_str += uppercase_first;
        }
        else {
            regex_str += lowercase_first;
        }
    }
    regex_str += "$";

    let r = regex::Regex::new(regex_str.as_str()).unwrap();
    if !r.is_match(context.name.as_str()) {
        return Err(regex_str);
    }
    return Ok(());
}
