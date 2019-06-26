use super::Path;

pub trait Query: Sized {
    /// Lookup for an element under the specified path.
    /// Returns an optional reference to the sought element.
    fn lookup<'v, 'p, P>(&'v self, path: P) -> Option<&'v Self>
    where
        P: Path<'p>;

    /// Takes the element under the specified path out of the queried node.
    /// Returns a tuple of two items:
    /// - optinal remainder of the queried node;
    /// - optinal taken away element.
    fn take<'p, P>(self, path: P) -> (Option<Self>, Option<Self>)
    where
        P: Path<'p>;

    /// Inserts an element into the queried node under the specified path.
    /// Returns None if inserted or Some(rejected_element) if could not perform insertion
    ///     (e.g. path leads to a child of a non-object sub-node).
    fn insert<'p, P>(&mut self, path: P, insertee: Self) -> Option<Self>
    where
        P: Path<'p>;
}
