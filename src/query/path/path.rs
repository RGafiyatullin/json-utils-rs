pub trait PathComponent<'a> {
    fn as_str_slice(&self) -> &str;
}

pub trait Path<'a> {
    type Item: PathComponent<'a>;
    type Iter: Iterator<Item = Self::Item> + 'a;
    fn path(self) -> Self::Iter;
}
