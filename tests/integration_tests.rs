extern crate rawncc;

#[test]
fn test_file_001_cpp() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

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
