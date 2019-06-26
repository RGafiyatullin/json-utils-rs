
use super::JsValueQuery;
use super::JsPath;

#[test]
fn empty_string_is_an_empty_path() {
    assert_eq!("".path().collect::<Vec<&str>>(), Vec::<&str>::new());
}

#[test]
pub fn lookup_test_01() {
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
    assert_eq!(object.lookup("two"), Some(&json!({
            "one": "two/one",
            "two": "two/two",
            "three": {
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            }
        })));
    assert_eq!(object.lookup("two/one"), Some(&json!("two/one")));
    assert_eq!(object.lookup("two/two"), Some(&json!("two/two")));
    assert_eq!(object.lookup("two/three"), Some(&json!({
                "one": "two/three/one",
                "two": ["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"]
            })));
    assert_eq!(object.lookup("two/three/one"), Some(&json!("two/three/one")));
    assert_eq!(object.lookup("two/three/two"), Some(&json!(["two/three/one[0]", "two/three/one[1]", "two/three/one[2]"])));
    assert_eq!(object.lookup("two/three/two/wat!?"), None);
}
