use tests_lib::add;
//-----------------------------------------
// WARN: In order for integration test to run; unit tests have to all pass successfully
// To force running itegrations tests despite unit tests not running
// $ cargo test --test integration_test
//-----------------------------------------
//
//
//-------------------------------------------
//In order to make a common module for the integration we have to do i with the current structure
//add the common module as a sperate file will confude the `cargo test` and display it as a testing
//modlule and count it despite havign not tests at all
//source: https://rust-book.cs.brown.edu/ch11-03-test-organization.html#submodules-in-integration-tests
//-------------------------------------------

mod commons;
#[test]
fn it_adds_two() {
    commons::setup();
    let result = add(2, 2);
    assert_eq!(result, 4);
}
