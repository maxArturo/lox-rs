use std::rc::Rc;

use super::Scope;

#[test]
fn basics() {
    let scope: Scope<String> = Scope::default();

    assert!(scope.get("foo").is_err());

    scope.define("foo", "bar".to_owned());
    assert!(scope.get("foo").is_ok());

    assert!(scope.assign("baz", "bar".to_owned()).is_err());

    scope.define("baz", "fff".to_owned());
    assert!(scope.assign("baz", "bar".to_owned()).is_ok());
    assert!(scope.get("baz").is_ok());
}

#[test]
fn scope_assignment() {
    let scope = Rc::new(Scope::default());
    let child = Scope::from_parent(Rc::clone(&scope));

    assert!(scope.get("foo").is_err());
    scope.define("foo", "bar".to_owned());
    assert!(scope.get("foo").is_ok_and(|str| str == "bar"));
    assert!(child.get("foo").is_ok_and(|str| str == "bar"));

    child.define("baz", "fff".to_owned());
    assert!(child.get("baz").is_ok_and(|str| str == "fff"));

    assert!(scope.get("baz").is_err());
    assert!(scope.assign("baz", "bar".to_owned()).is_err());

    assert!(child.assign("baz", "bar".to_owned()).is_ok());
    assert!(child.get("baz").is_ok());
}
