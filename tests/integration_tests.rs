extern crate rawncc;

use std::sync::Once;

static LOGGER: Once = Once::new();

fn test_setup() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    LOGGER.call_once(|| {
        env_logger::init();
    });
}

#[test]
fn test_file_001_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test001.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::VarContext>::new();
    let callback = |context| items.push(context);
    rawncc::parse_file(opts, callback);
    assert_eq!(22, items.len());
    assert_eq!(
        rawncc::VarContext {
            name: "UNNAMED_NAMESPACE".to_owned(),
            var_type: rawncc::VarContextType::Ptr,
            is_member: false,
            is_const: true,
            is_static: true,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 7,
                column: 13,
            }
        },
        items[0]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "the_const_string".to_owned(),
            var_type: rawncc::VarContextType::Ptr,
            is_member: false,
            is_const: true,
            is_static: true,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 10,
                column: 20,
            }
        },
        items[1]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "wtf".to_owned(),
            var_type: rawncc::VarContextType::Ptr,
            is_member: true,
            is_const: true,
            is_static: true,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 12,
                column: 22,
            }
        },
        items[2]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "THE_INT".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: true,
            is_const: true,
            is_static: true,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 23,
                column: 22,
            }
        },
        items[6]
    );
}

#[test]
fn test_file_002_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test002.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::VarContext>::new();
    let callback = |context| items.push(context);
    rawncc::parse_file(opts, callback);
    assert_eq!(5, items.len());
    assert_eq!(
        rawncc::VarContext {
            name: "TRANSLATION".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: false,
            is_const: true,
            is_static: true,
            src_location: rawncc::SrcLocation {
                file: "tests/test002.cpp".to_owned(),
                line_no: 6,
                column: 7,
            }
        },
        items[2]
    );
}

#[test]
fn test_fun_ra_nc() {
    test_setup();

    let result = rawncc::check_ra_nc(&rawncc::VarContext {
        name: "pClock".to_owned(),
        var_type: rawncc::VarContextType::Ptr,
        is_member: false,
        is_const: false,
        is_static: false,
        src_location: rawncc::SrcLocation {
            file: "foobar.cpp".to_owned(),
            line_no: 666,
            column: 42,
        },
    });
    assert!(result.is_ok());

    let result = rawncc::check_ra_nc(&rawncc::VarContext {
        name: "clockWork".to_owned(),
        var_type: rawncc::VarContextType::Value,
        is_member: false,
        is_const: false,
        is_static: false,
        src_location: rawncc::SrcLocation {
            file: "foobar.cpp".to_owned(),
            line_no: 666,
            column: 42,
        },
    });
    assert!(result.is_ok());
}
