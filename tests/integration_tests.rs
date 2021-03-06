extern crate rawncc;

use rawncc::{Callback, TCallback, VarContext};

use std::sync::Once;

static LOGGER: Once = Once::new();

fn test_setup() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    LOGGER.call_once(|| {
        env_logger::init();
    });
}

#[test]
fn test_vars_in_file_001_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test001.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<VarContext>::new();
    let mut callback = |context| items.push(context);
    rawncc::parse_file(opts, Callback::new(&mut callback));
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
                line_no: 5,
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
                line_no: 8,
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
                line_no: 10,
                column: 22,
            }
        },
        items[2]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "m_Int".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: true,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 18,
                column: 9,
            }
        },
        items[3]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "m_pInt".to_owned(),
            var_type: rawncc::VarContextType::Ptr,
            is_member: true,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 19,
                column: 10,
            }
        },
        items[4]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "m_rInt".to_owned(),
            var_type: rawncc::VarContextType::Ref,
            is_member: true,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 20,
                column: 10,
            }
        },
        items[5]
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
                line_no: 21,
                column: 22,
            }
        },
        items[6]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "the_const_d".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: false,
            is_const: true,
            is_static: true, // <- actuall 'internal' linkage
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 24,
                column: 14,
            }
        },
        items[7]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "the_const_unsigned".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: false,
            is_const: true,
            is_static: true, // <- actuall 'internal' linkage
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 25,
                column: 20,
            }
        },
        items[8]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "the_const_ref_d".to_owned(),
            var_type: rawncc::VarContextType::Ref,
            is_member: false,
            is_const: true,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 26,
                column: 15,
            }
        },
        items[9]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "the_const_ref_char".to_owned(),
            var_type: rawncc::VarContextType::Ref,
            is_member: false,
            is_const: true,
            is_static: true,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 27,
                column: 20,
            }
        },
        items[10]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "c".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: false,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 31,
                column: 10,
            }
        },
        items[11]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "b".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: false,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 32,
                column: 12,
            }
        },
        items[12]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "bb".to_owned(),
            var_type: rawncc::VarContextType::Ref,
            is_member: false,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 33,
                column: 14,
            }
        },
        items[13]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "d".to_owned(),
            var_type: rawncc::VarContextType::Ptr,
            is_member: false,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 34,
                column: 13,
            }
        },
        items[14]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "blah".to_owned(),
            var_type: rawncc::VarContextType::Ptr,
            is_member: false,
            is_const: true,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 35,
                column: 17,
            }
        },
        items[15]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "f".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: false,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 36,
                column: 11,
            }
        },
        items[16]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "g".to_owned(),
            var_type: rawncc::VarContextType::Ref,
            is_member: false,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 37,
                column: 12,
            }
        },
        items[17]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "h".to_owned(),
            var_type: rawncc::VarContextType::Ptr,
            is_member: false,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 38,
                column: 12,
            }
        },
        items[18]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "i".to_owned(),
            var_type: rawncc::VarContextType::Ref,
            is_member: false,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 39,
                column: 13,
            }
        },
        items[19]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "x".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: false,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 40,
                column: 9,
            }
        },
        items[20]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "test_001".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: false,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 41,
                column: 13,
            }
        },
        items[21]
    );
}

#[test]
fn test_vars_in_file_002_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test002.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::VarContext>::new();
    let mut callback = |context| items.push(context);
    rawncc::parse_file(opts, Callback::new(&mut callback));
    assert_eq!(3, items.len());
    assert_eq!(
        rawncc::VarContext {
            name: "number".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: true,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test002.cpp".to_owned(),
                line_no: 4,
                column: 13,
            }
        },
        items[0]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "result".to_owned(),
            var_type: rawncc::VarContextType::Value,
            is_member: true,
            is_const: false,
            is_static: false,
            src_location: rawncc::SrcLocation {
                file: "tests/test002.cpp".to_owned(),
                line_no: 5,
                column: 18,
            }
        },
        items[1]
    );
    assert_eq!(
        rawncc::VarContext {
            name: "TRANSLATION".to_owned(),
            var_type: rawncc::VarContextType::Array,
            is_member: false,
            is_const: true,
            is_static: true,
            src_location: rawncc::SrcLocation {
                file: "tests/test002.cpp".to_owned(),
                line_no: 6,
                column: 20,
            }
        },
        items[2]
    );
}

#[test]
fn test_cast_in_file_003_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test003.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::CastContext>::new();
    let mut callback = |context| items.push(context);
    rawncc::parse_file(opts, Callback::new(&mut callback));
    assert_eq!(1, items.len());
    assert_eq!(
        rawncc::CastContext {
            location: rawncc::SrcLocation {
                file: "tests/test003.cpp".to_owned(),
                line_no: 3,
                column: 12,
            }
        },
        items[0]
    );
}

#[test]
fn test_functions_in_file_001_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test001.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::FnContext>::new();
    let mut callback = |context| items.push(context);
    rawncc::parse_file(opts, Callback::new(&mut callback));
    assert_eq!(3, items.len());
    assert_eq!(
        rawncc::FnContext {
            name: "Temp".to_owned(),
            fn_type: rawncc::FnType::Ctor,
            location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 15,
                column: 5,
            }
        },
        items[0]
    );
    assert_eq!(
        rawncc::FnContext {
            name: "blah".to_owned(),
            fn_type: rawncc::FnType::Method,
            location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 16,
                column: 10,
            }
        },
        items[1]
    );
    assert_eq!(
        rawncc::FnContext {
            name: "main".to_owned(),
            fn_type: rawncc::FnType::Function,
            location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 29,
                column: 5,
            }
        },
        items[2]
    );
}

#[test]
fn test_functions_in_file_002_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test002.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::FnContext>::new();
    let mut callback = |context| items.push(context);
    rawncc::parse_file(opts, Callback::new(&mut callback));
    assert_eq!(1, items.len());
    assert_eq!(
        rawncc::FnContext {
            name: "getNumber".to_owned(),
            fn_type: rawncc::FnType::Function,
            location: rawncc::SrcLocation {
                file: "tests/test002.cpp".to_owned(),
                line_no: 1,
                column: 10,
            }
        },
        items[0]
    );
}

#[test]
fn test_functions_in_file_003_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test003.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::FnContext>::new();
    let mut callback = |context| items.push(context);
    rawncc::parse_file(opts, Callback::new(&mut callback));
    assert_eq!(2, items.len());
    assert_eq!(
        rawncc::FnContext {
            name: "test003".to_owned(),
            fn_type: rawncc::FnType::Function,
            location: rawncc::SrcLocation {
                file: "tests/test003.cpp".to_owned(),
                line_no: 1,
                column: 10,
            }
        },
        items[0]
    );
    assert_eq!(
        rawncc::FnContext {
            name: "test003_f".to_owned(),
            fn_type: rawncc::FnType::Function,
            location: rawncc::SrcLocation {
                file: "tests/test003.cpp".to_owned(),
                line_no: 6,
                column: 7,
            }
        },
        items[1]
    );
}

#[test]
fn test_complex_in_file_001_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test001.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::ComplexContext>::new();
    let mut callback = |context| items.push(context);
    rawncc::parse_file(opts, Callback::new(&mut callback));
    assert_eq!(1, items.len());
    assert_eq!(
        rawncc::ComplexContext {
            name: "Temp".to_owned(),
            c_type: rawncc::ComplexType::Struct,
            location: rawncc::SrcLocation {
                file: "tests/test001.cpp".to_owned(),
                line_no: 14,
                column: 8,
            }
        },
        items[0]
    );
}

#[test]
fn test_complex_in_file_001_hpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test001.hpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::ComplexContext>::new();
    let mut callback = |context| items.push(context);
    rawncc::parse_file(opts, Callback::new(&mut callback));
    assert_eq!(1, items.len());
    assert_eq!(
        rawncc::ComplexContext {
            name: "Test001".to_owned(),
            c_type: rawncc::ComplexType::Class,
            location: rawncc::SrcLocation {
                file: "tests/test001.hpp".to_owned(),
                line_no: 3,
                column: 7,
            }
        },
        items[0]
    );
}

#[test]
fn test_complex_in_file_002_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test002.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::ComplexContext>::new();
    let mut callback = |context| items.push(context);
    rawncc::parse_file(opts, Callback::new(&mut callback));
    assert_eq!(1, items.len());
    assert_eq!(
        rawncc::ComplexContext {
            name: "".to_owned(),
            c_type: rawncc::ComplexType::Struct,
            location: rawncc::SrcLocation {
                file: "tests/test002.cpp".to_owned(),
                line_no: 3,
                column: 5,
            }
        },
        items[0]
    );
}

#[test]
fn test_complex_in_file_003_cpp() {
    test_setup();

    let opts = rawncc::Options {
        debug: false,
        verbose: 0,
        input: std::path::PathBuf::from("tests/test003.cpp"),
        includes: vec![],
    };

    let mut items = Vec::<rawncc::ComplexContext>::new();
    let mut callback = |context| items.push(context);
    rawncc::parse_file(opts, Callback::new(&mut callback));
    assert_eq!(0, items.len());
}
