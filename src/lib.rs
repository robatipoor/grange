#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Range {
    pub current: i32,
    pub target: i32,
    pub step: i32,
}

#[macro_export]
macro_rules! range {
    (,,) => {{
        Range {
            current: 0,
            target: std::i32::MAX,
            step: 1,
        }
    }};
    (,,$end:expr) => {{
        if $end > 0 {
            Range {
                current: 0,
                target: $end,
                step: 1,
            }
        } else {
            Range {
                current: 0,
                target: $end,
                step: -1,
            }
        }
    }};
    (,,=$end:expr) => {{
        if $end > 0 {
            Range {
                current: 0,
                target: $end + 1,
                step: 1,
            }
        } else {
            Range {
                current: 0,
                target: $end - 1,
                step: -1,
            }
        }
    }};
    ($start:expr,,) => {{
        Range {
            current: $start,
            target: std::i32::MAX,
            step: 1,
        }
    }};
    ($start:expr,,$end:expr) => {{
        if $start < $end {
            Range {
                current: $start,
                target: $end,
                step: 1,
            }
        } else {
            Range {
                current: $start,
                target: $end,
                step: -1,
            }
        }
    }};
    ($start:expr,,=$end:expr) => {{
        if $start < $end {
            Range {
                current: $start,
                target: $end + 1,
                step: 1,
            }
        } else {
            Range {
                current: $start,
                target: $end - 1,
                step: -1,
            }
        }
    }};
    ($start:expr,,$step:expr,,$end:expr) => {{
        if ($start < $end && $step < 0) || ($start > $end && $step > 0) {
            panic!("invlid number");
        }
        Range {
            current: $start,
            target: $end,
            step: $step,
        }
    }};
    ($start:expr,,$step:expr,,=$end:expr) => {{
        if ($start < $end && $step < 0) || ($start > $end && $step > 0) {
            panic!("invlid number");
        }
        let p = if $step > 0 { 1 } else { -1 };
        Range {
            current: $start,
            target: $end + p,
            step: $step,
        }
    }};
    (,,$step:expr,,$end:expr) => {{
        if ($step > 0 && $end < 0) || ($step < 0 && $end > $step) {
            panic!("invlid number");
        }
        Range {
            current: 0,
            target: $end,
            step: $step,
        }
    }};
    (,,$step:expr,,=$end:expr) => {{
        if ($step > 0 && $end < 0) || ($step < 0 && $end > $step) {
            panic!("invlid number");
        }
        let p = if $step > 0 { 1 } else { -1 };
        Range {
            current: 0,
            target: $end + p,
            step: $step,
        }
    }};
}

impl Iterator for Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.step > 0 && self.current >= self.target
            || self.step < 0 && self.current <= self.target
        {
            None
        } else {
            let res = self.current;
            self.current += self.step;
            Some(res)
        }
    }
}

impl Range {
    #[allow(dead_code)]
    fn parse(pattern: &str) -> Result<Range, String> {
        let pattern: i32 = pattern.parse().unwrap(); // TODO use Regex for parse string
        Ok(range!(,,pattern))
    }
}
