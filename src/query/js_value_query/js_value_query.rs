use super::JsPath;

pub trait JsValueQuery {
    fn lookup<'v, 'p, P>(&'v self, path: P) -> Option<&'v Self>
    where
        P: JsPath<'p>;
}
