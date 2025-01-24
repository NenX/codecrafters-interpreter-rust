
pub trait ResolverWalk<T> {
    fn resolve(&mut self, value: &T);
}
