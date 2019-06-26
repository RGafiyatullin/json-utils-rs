use super::JsPath;
use super::JsPathComponent;

impl<'a> JsPathComponent<'a> for &'a str {
    fn as_str_slice(&self) -> &str {
        self
    }
}

impl<'a> JsPath<'a> for &'a str {
    type Item = &'a str;
    type Iter = StrSliceIter<'a>;
    fn path(self) -> Self::Iter {
        if self.is_empty() { StrSliceIter::Empty }
        else { StrSliceIter::NonEmpty(self.split("/")) }
    }
}

pub enum StrSliceIter<'a> {
    Empty,
    NonEmpty(std::str::Split<'a, &'a str>),
}

impl<'a> Iterator for StrSliceIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            StrSliceIter::Empty => None,
            StrSliceIter::NonEmpty(ref mut inner) => inner.next(),
        }
    }
}
