extern crate rust_hyper_server_template;

#[tokio::test]
async fn test() {
    rust_hyper_server_template::setup_test_environment();

    assert_eq!(1, 1);
}
