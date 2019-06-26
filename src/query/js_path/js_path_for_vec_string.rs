use super::JsPath;
use super::JsPathComponent;

impl<'a> JsPathComponent<'a> for &'a String {
    fn as_str_slice(&self) -> &str {
        self
    }
}

impl<'a> JsPath<'a> for &'a Vec<String> {
    type Item = &'a String;
    type Iter = std::slice::Iter<'a, String>;

    fn path(self) -> Self::Iter {
        self.iter()
    }
}
