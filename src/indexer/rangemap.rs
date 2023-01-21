use crate::{BuildIndexer, Indexer};
use ordered_float::OrderedFloat;
use rangemap::RangeMap;

#[derive(Default)]
pub struct BuildRangeMapIndexer;

pub struct RangeMapIndexer {
    min: OrderedFloat<f64>,
    max: OrderedFloat<f64>,
    map: RangeMap<OrderedFloat<f64>, usize>,
}

impl Indexer<f64, usize> for RangeMapIndexer {
    fn index(&self, v: f64) -> usize {
        let v = OrderedFloat(v).clamp(self.min, self.max);
        *self.map.get(&v).unwrap()
    }
}

impl BuildIndexer<f64, usize> for BuildRangeMapIndexer {
    type Indexer = RangeMapIndexer;
    fn build_indexer<C>(&self, min: f64, max: f64, ticks: &[C]) -> Self::Indexer {
        let min = min.into();
        let max = max.into();
        let mut map = rangemap::RangeMap::new();
        let idx_per_step: OrderedFloat<f64> =
            OrderedFloat::<f64>::from((ticks.len() - 1) as f64) / (max - min);
        for (i, _) in ticks.iter().enumerate() {
            let start: OrderedFloat<_> = min + OrderedFloat::<f64>::from(i as f64) / idx_per_step;
            let end: OrderedFloat<_> =
                min + (OrderedFloat::<f64>::from((i + 1) as f64) / idx_per_step);
            println!("{} {}", min, max);
            println!("{idx_per_step}");
            println!(
                "{:?}",
                OrderedFloat::<f64>::from(i as f64 + 1.0) / idx_per_step
            );
            println!("{:?} {:?}", start, end);
            map.insert(start..end, i);
        }
        RangeMapIndexer { min, max, map }
    }
}
