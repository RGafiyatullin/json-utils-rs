use crate::query::Path;
use crate::query::Query;

#[test]
fn string_ref_as_path() {
    let object = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        },
    });

    let path: String = "two/three".to_owned();

    assert_eq!(
        object.lookup(&path),
        Some(&json!({
            "one": "two/three/one",
            "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
        }))
    );
}

#[test]
fn empty_string_is_an_empty_path() {
    assert_eq!("".path().collect::<Vec<&str>>(), Vec::<&str>::new());
}

#[test]
fn lookup_test_01() {
    let object = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        },
    });

    assert_eq!(object.lookup(""), Some(&object));

    assert_eq!(object.lookup("one"), Some(&json!("one")));
    assert_eq!(object.lookup("one/two"), None);
    assert_eq!(
        object.lookup("two"),
        Some(&json!({
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        }))
    );
    assert_eq!(object.lookup("two/one"), Some(&json!("two/one")));
    assert_eq!(object.lookup("two/two"), Some(&json!("two/two")));
    assert_eq!(
        object.lookup("two/three"),
        Some(&json!({
            "one": "two/three/one",
            "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
        }))
    );
    assert_eq!(
        object.lookup("two/three/one"),
        Some(&json!("two/three/one"))
    );
    assert_eq!(
        object.lookup("two/three/two"),
        Some(&json!([
            "two/three/one[0]",
            "two/three/one[1]",
            "two/three/one[2]"
        ]))
    );
    assert_eq!(object.lookup("two/three/two/wat!?"), None);
}

#[test]
fn take_test_01() {
    let object = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        },
    });

    let (object_opt, taken_opt) = object.clone().take("");

    assert_eq!(object_opt, None);
    assert_eq!(taken_opt, Some(object));
}

#[test]
fn take_test_02() {
    let object = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        },
    });

    let (object_opt, taken_opt) = object.clone().take("zero");
    assert_eq!(object_opt, Some(object));
    assert_eq!(taken_opt, None);
}

#[test]
fn take_test_03() {
    let object = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        },
    });

    let (object_opt, taken_opt) = object.take("two");
    assert_eq!(
        object_opt,
        Some(json!({
            "one": "one"
        }))
    );
    assert_eq!(
        taken_opt,
        Some(json!({
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        }))
    );
}

#[test]
fn take_test_04() {
    let object = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        },
    });

    let (object_opt, taken_opt) = object.take("two/three");
    assert_eq!(
        object_opt,
        Some(json!(
            {
            "one": "one",
            "two": {
                "one": "two/one",
                "two": "two/two",
            },
        }
        ))
    );
    assert_eq!(
        taken_opt,
        Some(json!({
            "one": "two/three/one",
            "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
        }))
    );
}

#[test]
fn insert_test_01() {
    let mut object = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        },
    });
    let insertee = json!("two/three");
    let expected = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": "two/three"
        },
    });

    assert_eq!(object.insert("two/three", insertee), None);

    assert_eq!(object, expected);
}

#[test]
fn insert_test_02() {
    let mut object = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        },
    });
    let insertee = json!("two/three/three");
    let expected = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"],
                "three": "two/three/three",
            }
        },
    });

    assert_eq!(object.insert("two/three/three", insertee), None);

    assert_eq!(object, expected);
}

#[test]
fn insert_test_03() {
    let original_object = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        },
    });
    let mut object = original_object.clone();
    let original_insertee = json!(null);
    let insertee = original_insertee.clone();

    assert_eq!(object.insert("one/zero", insertee), Some(original_insertee));
    assert_eq!(object, original_object);
}

#[test]
fn insert_test_04() {
    let mut object = json!({
        "one": "one",
        "two": {
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        },
    });
    let original_insertee = json!(null);
    let insertee = original_insertee.clone();

    assert_eq!(object.insert("", insertee), None);
    assert_eq!(object, original_insertee);
}
