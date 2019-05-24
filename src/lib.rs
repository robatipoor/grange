mod range_macro;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Range {
    pub current: i32,
    pub target: i32,
    pub step: i32,
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
#[allow(dead_code)]
impl Range {
    fn is_empty() {
        unimplemented!();
    }
    fn contains() {
        unimplemented!();
    }
}
