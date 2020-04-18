extern crate rust_hyper_server_template;
use std::collections::HashMap;

#[tokio::test]
async fn test() {
    rust_hyper_server_template::setup_test_env!();

    assert_eq!(1, 1);
}
