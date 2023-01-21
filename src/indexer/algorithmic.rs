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
        // ticks.len() - 1: 7, max: 16, min: 1
        // (max - min): 15.
        // 16 / 7:
        // 16
        let max_index = (ticks.len() - 1) as f64;
        let idx_per_step_from_max = max_index / max;
        let idx_per_step_from_min = max_index / min;
        let idx_per_step = idx_per_step_from_max - min;
        println!(
            "{} {} {} {} {} {}",
            min,
            max,
            idx_per_step_from_min,
            idx_per_step_from_max,
            idx_per_step,
            ((ticks.len() - 1) as f64 / (max - min))
        );
        AlgorithmicIndexer {
            min,
            max,
            idx_per_step,
        }
    }
}
