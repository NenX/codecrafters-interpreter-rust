use crate::evaluator::Evaluator;

pub trait ResolverWalk<T> {
    fn resolve(&mut self, value: &T);
}
