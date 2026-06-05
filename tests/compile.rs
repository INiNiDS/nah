#[test]
fn pass_dev_mode() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/fixtures/pass.rs");
}

#[test]
fn fail_deploy_mode() {
    unsafe {
        std::env::set_var("DEPLOY", "1");
    }
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/fixtures/deploy_blocked.rs");
}
