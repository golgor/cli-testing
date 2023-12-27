use cli_interface;

#[test]
fn integration_test_testare() {
    assert_eq!(15, cli_interface::funcs::testare(5));
}
