#[cfg(test)]
use crate::Env;

#[test]
fn basics() {
    let mut env: Env<String> = Env::new();
    assert!(env.get("foo").is_err());

    env.define("foo", "bar".to_owned());
    assert!(env.get("foo").is_ok());

    assert!(env.assign("baz", "bar".to_owned()).is_err());

    env.define("baz", "fff".to_owned());
    assert!(env.assign("baz", "bar".to_owned()).is_ok());
    assert!(env.get("baz").is_ok());
}

#[test]
fn stack_blowout() {
    let mut env: Env<String> = Env::new();
    for _i in 1..10_000_000 {
        env.open_scope();
    }
}

#[test]
fn scope_assignment() {
    let mut env: Env<String> = Env::new();
    env.define("foo", "bar".to_owned());
    assert!(env.get("foo").is_ok_and(|str| str == "bar"));

    env.open_scope();
    assert!(env.get("foo").is_ok());
    assert!(env.get("baz").is_err());
    env.define("baz", "bar".to_owned());
    assert!(env.get("baz").is_ok());

    assert!(env.assign("foo", "cow".to_owned()).is_ok());
    env.close_scope();
    assert!(env.get("foo").is_ok_and(|str| str == "cow"));
}
