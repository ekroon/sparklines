const TICKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
const MIDDLE_IDX: usize = TICKS.len() / 2;

pub fn spark(data: &[f64]) -> String {
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
                result.push(TICKS[MIDDLE_IDX]);
            })
        } else {
            let f = (max - min) / (TICKS.len() - 1) as f64;
            data.iter().for_each(|v| {
                let idx = ((v - min) / f) as usize;
                result.push(TICKS[idx]);
            });
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graphs_data() {
        assert_eq!(spark(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]), "▁▂▃▄▅▆▇█");
    }

    #[test]
    fn equalizes_at_midtier_when_all_equal() {
        assert_eq!(spark(&[1.0, 1.0, 1.0, 1.0]), "▅▅▅▅")
    }

    #[test]
    fn handles_empty_data() {
        assert_eq!(spark(&[]), "");
    }

    #[test]
    fn handles_single_element() {
        assert_eq!(spark(&[1.0]), "▅");
    }
}
