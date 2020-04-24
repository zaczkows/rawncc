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
    assert_eq!(19, items.len());
}
