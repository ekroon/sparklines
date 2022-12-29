pub mod algorithmic;

pub trait Indexer<T, S> {
    fn index(&self, v: T) -> S;
}

pub trait BuildIndexer<T, S> {
    type Indexer: Indexer<T, S>;
    fn build_indexer<C>(&self, min: T, max: T, ticks: &[C]) -> Self::Indexer;
}
