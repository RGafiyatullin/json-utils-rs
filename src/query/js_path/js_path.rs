pub trait JsPathComponent<'a> {
    fn as_str_slice(&self) -> &str;
}

pub trait JsPath<'a> {
    type Item: JsPathComponent<'a>;
    type Iter: Iterator<Item = Self::Item> + 'a;
    fn path(self) -> Self::Iter;
}
