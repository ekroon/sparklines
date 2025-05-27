use crate::{BuildIndexer, Indexer};
use ordered_float::OrderedFloat;
use rangemap::RangeMap;

#[derive(Default)]
pub struct BuildRangemapIndexer;

pub struct RangemapIndexer {
    min: OrderedFloat<f64>,
    max: OrderedFloat<f64>,
    map: RangeMap<OrderedFloat<f64>, usize>,
}

fn next_up(v: f64) -> f64 {
    if v.is_nan() || v == f64::INFINITY {
        v
    } else if v == 0.0f64 && v.is_sign_negative() {
        f64::MIN_POSITIVE
    } else if v >= 0.0 {
        f64::from_bits(v.to_bits() + 1)
    } else {
        f64::from_bits(v.to_bits() - 1)
    }
}

impl Indexer<f64, usize> for RangemapIndexer {
    fn index(&self, v: f64) -> usize {
        let value = OrderedFloat(v).max(self.min).min(self.max);
        *self.map.get(&value).expect("value not in range")
    }
}

impl BuildIndexer<f64, usize> for BuildRangemapIndexer {
    type Indexer = RangemapIndexer;

    fn build_indexer<C>(&self, min: f64, max: f64, ticks: &[C]) -> Self::Indexer {
        let step = (max - min) / (ticks.len() as f64 - 1.0);
        let mut map = RangeMap::new();
        let mut start = min;
        for idx in 0..ticks.len() {
            let mut end = if idx == ticks.len() - 1 {
                max
            } else {
                min + step * (idx as f64 + 1.0)
            };
            if idx == ticks.len() - 1 {
                end = next_up(end);
            }
            map.insert(OrderedFloat(start)..OrderedFloat(end), idx);
            start = end;
        }
        RangemapIndexer {
            min: OrderedFloat(min),
            max: OrderedFloat(max),
            map,
        }
    }
}
