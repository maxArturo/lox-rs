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

    assert!(scope.get("foo").is_err());
    scope.define("foo", "bar".to_owned());
    assert!(scope.get("foo").is_ok_and(|str| str == "bar"));
}

#[test]
fn ancestry() {
    let grandparent: Rc<Scope<bool>> = Rc::new(Scope::default());
    grandparent.define("grandy", true);

    let parent = Scope::from_parent(Rc::clone(&grandparent));

    parent.define("parent", true);
    let child = Scope::from_parent(Rc::clone(&parent));

    child.define("child", true);

    assert!(child.get_at(0, "child").is_ok_and(|el| el));
    assert!(child.get_at(1, "parent").is_ok_and(|el| el));
    assert!(child.get_at(2, "grandy").is_ok_and(|el| el));

    assert!(grandparent.assign_at(33, "foo", true).is_err());
    assert!(grandparent.assign_at(1, "bar", true).is_err());
    assert!(grandparent.assign_at(0, "works", true).is_ok());

    assert!(child.get_at(2, "works").is_ok_and(|el| el));
}
