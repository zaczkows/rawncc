use rawncc;

fn recurse_check(entity: &Vec<clang::Entity>, file_location: &str) {
    entity
        .iter()
        .filter(|x| {
            let loc = x.get_location();
            if let Some(l) = loc {
                if let Some(f) = l.get_file_location().file {
                    return f.get_path().to_str().unwrap() == file_location;
                }
            }

            false
        })
        .for_each(|x| {
            log::debug!("Parsing {:?}", x);
            recurse_check(&x.get_children(), file_location);
        });
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
    log::debug!("translation unit: {:?}", &tu);
    let entity = tu.get_entity();
    log::debug!("entity for TU: {:?}", &entity);
    if let Some(l) = entity.get_language() {
        log::debug!("language for TU is {:?}", l);
    }

    let kind = entity.get_kind();
    log::debug!("parsed {:?} kind", &kind);

    let file_location = options.input.to_str().expect("Invalid filename");
    recurse_check(&entity.get_children(), &file_location);
}
