use crate::{BuildIndexer, Indexer};

#[derive(Default)]
pub struct BuildAlgorithmicIndexer;

pub struct AlgorithmicIndexer {
    min: f64,
    max: f64,
    idx_per_step: f64,
}

impl Indexer<f64, usize> for AlgorithmicIndexer {
    fn index(&self, v: f64) -> usize {
        let v = v.clamp(self.min, self.max);
        ((v - self.min) * self.idx_per_step).round() as usize
    }
}

impl BuildIndexer<f64, usize> for BuildAlgorithmicIndexer {
    type Indexer = AlgorithmicIndexer;
    fn build_indexer<C>(&self, min: f64, max: f64, ticks: &[C]) -> Self::Indexer {
        AlgorithmicIndexer {
            min,
            max,
            idx_per_step: (ticks.len() - 1) as f64 / (max - min),
        }
    }
}
