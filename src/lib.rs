//! The sparklines crate provides a simple way to generate sparklines.

/// Default ticks for create a string sparkline.
/// ```
/// assert_eq!(sparklines::TICKS, ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█']);
/// ```
pub const TICKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// `StringSparkline` is a struct that can be used to create a string sparkline.
pub struct StringSpark<'a> {
    ticks: &'a [char],
    middle_idx: usize,
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
            ticks: &ticks,
            middle_idx: ticks.len() / 2,
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
        let mut min = None;
        let mut max = None;
        for v in data {
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
        if let (Some(min), Some(max)) = (min, max) {
            if min.eq(max) {
                data.iter().for_each(|_| {
                    result.push(self.ticks[self.middle_idx]);
                })
            } else {
                let idx_per_step = (self.ticks.len() - 1) as f64 / (max - min);
                data.iter().for_each(|v| {
                    let idx = ((v - min) * idx_per_step).round() as usize;
                    result.push(self.ticks[idx]);
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
    fn test_spark_fn() {
        assert_eq!(spark(&[1.0, 2.0, 3.0]), "▁▅█");
    }
}
