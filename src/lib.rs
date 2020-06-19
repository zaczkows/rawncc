mod callback;
mod cast_context;
mod complex_context;
mod fn_context;
mod opts;
mod srclocation;
mod varcontext;

pub use callback::{Callback, TCallback};
pub use cast_context::CastContext;
pub use complex_context::{ComplexContext, ComplexType};
pub use fn_context::{FnContext, FnType};
pub use opts::Options;
pub use srclocation::SrcLocation;
pub use varcontext::{VarContext, VarContextType};

#[macro_use]
extern crate lazy_static;

fn get_clang() -> &'static clang::Clang {
    lazy_static! {
        static ref CLANG: clang::Clang = clang::Clang::new().expect("Failed to create basic clang object");
    }

    &CLANG
}

pub fn parse_file(options: Options, mut callback: Callback) {
    log::debug!("Using {}", clang::get_version());
    let c = get_clang();
    let i = clang::Index::new(&c, false, options.verbose > 0);
    let mut cpp_arguments = vec!["-x", "c++", "-std=c++11", "-fsyntax-only"];
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
        return;
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
                return clang::EntityVisitResult::Continue;
            }
        }

        if options.debug {
            log::debug!("Entity item: {:?}", &entity);
        }

        let entity_kind = entity.get_kind();
        if callback.fun.is_some() && fn_context::is_fn_type(&entity_kind).is_some() {
            (callback.fun.as_mut().unwrap())(FnContext::from(&entity));
            return clang::EntityVisitResult::Recurse;
        }

        if callback.complex.is_some() && complex_context::get_complex_type(&entity_kind).is_some() {
            (callback.complex.as_mut().unwrap())(ComplexContext::from(&entity));
            return clang::EntityVisitResult::Recurse;
        }

        match entity.get_kind() {
            clang::EntityKind::VarDecl | clang::EntityKind::FieldDecl => {
                if callback.var.is_some() {
                    (callback.var.as_mut().unwrap())(VarContext::from(&entity, &parent));
                }
                return clang::EntityVisitResult::Continue;
            }
            clang::EntityKind::CStyleCastExpr => {
                if callback.cast.is_some() {
                    (callback.cast.as_mut().unwrap())(CastContext {
                        location: SrcLocation::from(&entity),
                    });
                }
                return clang::EntityVisitResult::Continue;
            }
            clang::EntityKind::ConstAttr => {
                log::debug!("Found const attr: {:?}", &entity);
            }
            _ => (),
        }

        clang::EntityVisitResult::Recurse
    });
}

pub fn check_ra_nc_var(context: &VarContext) -> Result<(), String> {
    let mut regex_str = String::from("^");
    if context.is_const || context.is_static {
        regex_str += "[A-Z][A-Z0-9]+(_[A-Z0-9]+)*";
    } else {
        let uppercase_first = "([A-Z][a-z0-9]+)+";
        let lowercase_first = "[a-z][a-z0-9]*([A-Z][a-z0-9]+)*";
        match (&context.var_type, context.is_member) {
            (VarContextType::Value, false) => regex_str += lowercase_first,
            (VarContextType::Value, true) => {
                regex_str += "m_";
                regex_str += uppercase_first;
            }
            (VarContextType::Ptr, false) => {
                regex_str += "p";
                regex_str += uppercase_first;
            }
            (VarContextType::Ptr, true) => {
                regex_str += "m_";
                regex_str += "p";
                regex_str += uppercase_first;
            }
            (VarContextType::Ref, false) => {
                regex_str += "r";
                regex_str += uppercase_first;
            }
            (VarContextType::Ref, true) => {
                regex_str += "m_";
                regex_str += "r";
                regex_str += uppercase_first;
            }
            (VarContextType::Array, false) => {
                regex_str += "r";
                regex_str += lowercase_first;
            }
            (VarContextType::Array, true) => {
                regex_str += "m_";
                regex_str += uppercase_first;
            }
        }
    }
    regex_str += "$";

    let r = regex::Regex::new(regex_str.as_str()).unwrap();
    if !r.is_match(context.name.as_str()) {
        return Err(regex_str);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fun_ra_nc_var_simple() {
        fn check_var(name: &str, var_type: VarContextType) -> Result<(), String> {
            check_ra_nc_var(&VarContext {
                name: name.to_owned(),
                var_type,
                is_member: false,
                is_const: false,
                is_static: false,
                src_location: SrcLocation {
                    file: "foobar.cpp".to_owned(),
                    line_no: 666,
                    column: 42,
                },
            })
        }

        assert!(check_var("clock", VarContextType::Value).is_ok());
        assert!(check_var("clockType", VarContextType::Value).is_ok());
        assert!(check_var("clock007Type", VarContextType::Value).is_ok());
        assert!(check_var("clockTypeMe", VarContextType::Value).is_ok());
        assert!(check_var("pClockTypeMe", VarContextType::Value).is_ok());
        assert!(check_var("rClockTypeMe", VarContextType::Value).is_ok());
        assert!(check_var("ClockTypeMe", VarContextType::Value).is_err());
        assert!(check_var("clock_type_me", VarContextType::Value).is_err());
        assert!(check_var("666Clock", VarContextType::Value).is_err());

        assert!(check_var("pClock", VarContextType::Ptr).is_ok());
        assert!(check_var("pClockWork", VarContextType::Ptr).is_ok());
        assert!(check_var("pClock666Work", VarContextType::Ptr).is_ok());
        assert!(check_var("clockWork", VarContextType::Ptr).is_err());
        assert!(check_var("ClockWork", VarContextType::Ptr).is_err());
        assert!(check_var("pClock_Work", VarContextType::Ptr).is_err());
        assert!(check_var("p_clock_work", VarContextType::Ptr).is_err());

        assert!(check_var("rClock", VarContextType::Ref).is_ok());
        assert!(check_var("rClockWork", VarContextType::Ref).is_ok());
        assert!(check_var("rClock666Work", VarContextType::Ref).is_ok());
        assert!(check_var("clockWork", VarContextType::Ref).is_err());
        assert!(check_var("ClockWork", VarContextType::Ref).is_err());
        assert!(check_var("rClock_Work", VarContextType::Ref).is_err());
        assert!(check_var("r_clock_work", VarContextType::Ref).is_err());
    }

    #[test]
    fn test_fun_ra_nc_var_member() {
        fn check_var(name: &str, var_type: VarContextType) -> Result<(), String> {
            assert!(check_ra_nc_var(&VarContext {
                name: name.to_owned(),
                var_type: var_type.clone(),
                is_member: true,
                is_const: false,
                is_static: false,
                src_location: SrcLocation {
                    file: "foobar.cpp".to_owned(),
                    line_no: 666,
                    column: 42,
                },
            })
            .is_err());

            check_ra_nc_var(&VarContext {
                name: String::from("m_") + name,
                var_type,
                is_member: true,
                is_const: false,
                is_static: false,
                src_location: SrcLocation {
                    file: "foobar.cpp".to_owned(),
                    line_no: 666,
                    column: 42,
                },
            })
        }

        assert!(check_var("clock", VarContextType::Value).is_err());
        assert!(check_var("clockType", VarContextType::Value).is_err());
        assert!(check_var("clock007Type", VarContextType::Value).is_err());
        assert!(check_var("clockTypeMe", VarContextType::Value).is_err());
        assert!(check_var("pClockTypeMe", VarContextType::Value).is_err());
        assert!(check_var("rClockTypeMe", VarContextType::Value).is_err());
        assert!(check_var("ClockTypeMe", VarContextType::Value).is_ok());
        assert!(check_var("clock_type_me", VarContextType::Value).is_err());

        assert!(check_var("pClock", VarContextType::Ptr).is_ok());
        assert!(check_var("pClockWork", VarContextType::Ptr).is_ok());
        assert!(check_var("pClock666Work", VarContextType::Ptr).is_ok());
        assert!(check_var("clockWork", VarContextType::Ptr).is_err());
        assert!(check_var("ClockWork", VarContextType::Ptr).is_err());
        assert!(check_var("pClock_Work", VarContextType::Ptr).is_err());
        assert!(check_var("p_clock_work", VarContextType::Ptr).is_err());
        assert!(check_var("p_clock_work", VarContextType::Ptr).is_err());

        assert!(check_var("rClock", VarContextType::Ref).is_ok());
        assert!(check_var("rClockWork", VarContextType::Ref).is_ok());
        assert!(check_var("rClock666Work", VarContextType::Ref).is_ok());
        assert!(check_var("clockWork", VarContextType::Ref).is_err());
        assert!(check_var("ClockWork", VarContextType::Ref).is_err());
        assert!(check_var("rClock_Work", VarContextType::Ref).is_err());
        assert!(check_var("r_clock_work", VarContextType::Ref).is_err());
    }

    #[test]
    fn test_fun_ra_nc_var_const() {
        fn check_var(name: &str, var_type: VarContextType) -> Result<(), String> {
            assert!(check_ra_nc_var(&VarContext {
                name: name.to_owned(),
                var_type: var_type.clone(),
                is_member: false,
                is_const: true,
                is_static: false,
                src_location: SrcLocation {
                    file: "foobar.cpp".to_owned(),
                    line_no: 666,
                    column: 42,
                },
            })
            .is_err());

            let mut n = String::from(name);
            n.make_ascii_uppercase();
            check_ra_nc_var(&VarContext {
                name: n,
                var_type,
                is_member: false,
                is_const: true,
                is_static: false,
                src_location: SrcLocation {
                    file: "foobar.cpp".to_owned(),
                    line_no: 666,
                    column: 42,
                },
            })
        }

        assert!(check_var("clock", VarContextType::Value).is_ok());
        assert!(check_var("clockType", VarContextType::Value).is_ok());
        assert!(check_var("clock007Type", VarContextType::Value).is_ok());
        assert!(check_var("ClockTypeMe", VarContextType::Value).is_ok());
        assert!(check_var("clock_type_me", VarContextType::Value).is_ok());
        assert!(check_var("666_clock_type_me", VarContextType::Value).is_err());
    }
}
