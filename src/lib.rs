//! The sparklines crate provides a simple way to generate sparklines.

use crate::indexer::algorithmic::BuildAlgorithmicIndexer;
use crate::indexer::{BuildIndexer, Indexer};

mod indexer;

/// Default ticks for create a string sparkline.
/// ```
/// assert_eq!(sparklines::TICKS, ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█']);
/// ```
pub const TICKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// `StringSparkline` is a struct that can be used to create a string sparkline.
pub struct StringSpark<'a, I = BuildAlgorithmicIndexer>
where
    I: BuildIndexer<f64, usize>,
{
    min: Option<f64>,
    max: Option<f64>,
    ticks: &'a [char],
    middle_idx: usize,
    build_indexer: I,
}

impl<'a> StringSpark<'a> {
    /// Create a new `SparkLines` instance.
    ///
    /// # Examples
    /// ```
    /// # use sparklines::TICKS;
    ///
    /// let spark = sparklines::StringSpark::new(&TICKS);
    /// assert_eq!(spark.spark(&[1.0,2.0,3.0]), "▁▅█");
    ///
    /// let spark = sparklines::StringSpark::new(&['a','b','c']);
    /// assert_eq!(spark.spark(&[1.0,2.0,3.0]), "abc");
    /// ```
    pub fn new(ticks: &'a [char]) -> Self {
        Self {
            min: None,
            max: None,
            ticks,
            middle_idx: ticks.len() / 2,
            build_indexer: Default::default(),
        }
    }

    /// Create a new `SparkLines` instance.
    ///
    /// # Examples
    /// ```
    /// # use sparklines::TICKS;
    ///
    /// let spark = sparklines::StringSpark::new_with_min_max(&TICKS, 2.0, 3.0);
    /// assert_eq!(spark.spark(&[1.0,2.0,3.0,4.0]), "▁▁██");
    ///
    /// let spark = sparklines::StringSpark::new_with_min_max(&TICKS, 1.0, 3.0);
    /// assert_eq!(spark.spark(&[0.0,2.0,300.0]), "▁▅█");
    /// ```
    pub fn new_with_min_max(ticks: &'a [char], min: f64, max: f64) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
            ticks,
            middle_idx: ticks.len() / 2,
            build_indexer: Default::default(),
        }
    }

    /// Convert a slice of `f64` values into a String representing a sparkline.
    ///
    /// # Example
    /// ```
    /// assert_eq!(sparklines::StringSpark::new(&sparklines::TICKS).spark(&[1.0,2.0,3.0]), "▁▅█");
    /// ```
    pub fn spark(&self, data: &[f64]) -> String {
        let mut result = String::with_capacity(data.len() * 4);
        let mut min: Option<&f64> = self.min.as_ref();
        let mut max: Option<&f64> = self.max.as_ref();
        if min.is_none() || max.is_none() {
            for v in data {
                if v.is_nan() {
                    continue;
                }
                if let Some(m) = min {
                    if v < m {
                        min = Some(v);
                    }
                } else {
                    min = Some(v);
                }
                if let Some(m) = max {
                    if v > m {
                        max = Some(v);
                    }
                } else {
                    max = Some(v);
                }
            }
        }
        if let (Some(min), Some(max)) = (min, max) {
            if min.eq(max) {
                data.iter().for_each(|_| {
                    result.push(self.ticks[self.middle_idx]);
                })
            } else {
                let indexer = self.build_indexer.build_indexer(*min, *max, self.ticks);
                data.iter().for_each(|v| {
                    if !v.is_nan() {
                        result.push(self.ticks[indexer.index(*v)]);
                    }
                });
            }
        }
        result
    }
}

impl Default for StringSpark<'_> {
    fn default() -> Self {
        StringSpark::new(&TICKS)
    }
}

/// Converts a slice of `f64` to a `String` representing a sparkline .
///
/// # Example
/// ```
/// # use sparklines::spark;
/// assert_eq!(spark(&[1.0,2.0,3.0]), "▁▅█");
/// ```
pub fn spark(data: &[f64]) -> String {
    StringSpark::default().spark(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0] => "▁▁▂▂▃▃▄▄▅▅▆▆▇▇██")]
    #[test_case(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0] => "▁▂▃▄▅▆▇█")]
    #[test_case(&[1.0, 1.0, 1.0, 1.0] => "▅▅▅▅")]
    #[test_case(&[1.0 ] => "▅")]
    #[test_case(&[] => "")]
    fn test_spark(data: &[f64]) -> String {
        let spark = StringSpark::new(&TICKS);
        spark.spark(data)
    }

    #[test]
    fn test_default() {
        let spark = StringSpark::default();
        assert_eq!(spark.spark(&[1.0, 2.0, 3.0]), "▁▅█");
    }

    #[test]
    fn test_non_default() {
        let spark = StringSpark::new(&['a', 'b', 'c']);
        assert_eq!(spark.spark(&[1.0, 2.0, 3.0]), "abc");
    }

    #[test]
    fn test_nan() {
        let spark = StringSpark::default();
        assert_eq!(spark.spark(&[f64::NAN, 1.0, 2.0, f64::NAN, 3.0]), "▁▅█");
    }

    #[ignore]
    #[test]
    fn test_infinite() {
        let spark = StringSpark::default();
        assert_eq!(
            spark.spark(&[f64::NEG_INFINITY, 0.0, f64::INFINITY,]),
            "▁▅█"
        );
    }

    #[test]
    fn test_spark_fn() {
        assert_eq!(spark(&[1.0, 2.0, 3.0]), "▁▅█");
    }
}
