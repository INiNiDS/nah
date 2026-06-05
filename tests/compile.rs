use std::sync::OnceLock;

static DEPLOY_MUTEX: OnceLock<std::sync::Mutex<()>> = OnceLock::new();

fn with_deploy<F>(value: Option<&str>, f: &F)
where
    F: Fn(),
{
    let _guard = DEPLOY_MUTEX
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();
    let old = std::env::var("DEPLOY").ok();
    match value {
        Some(v) => unsafe { std::env::set_var("DEPLOY", v) },
        None => unsafe { std::env::remove_var("DEPLOY") },
    }
    f();
    match old {
        Some(v) => unsafe { std::env::set_var("DEPLOY", v) },
        None => unsafe { std::env::remove_var("DEPLOY") },
    }
}

#[test]
fn pass_dev_mode() {
    with_deploy(None, &|| {
        let tests = trybuild::TestCases::new();
        tests.pass("tests/fixtures/pass.rs");
    });
}

#[test]
fn fail_deploy_mode() {
    with_deploy(Some("1"), &|| {
        let tests = trybuild::TestCases::new();
        tests.compile_fail("tests/fixtures/deploy_blocked.rs");
    });
}
