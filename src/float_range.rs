use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct FloatRange {
    pub start: f64,
    pub stop: f64,
    pub step: f64,
}

impl FloatRange {
    pub fn with_step(self, new_step: f64) -> Self {
        FloatRange {
            step: new_step,
            ..self
        }
    }
}

impl<T> From<Range<T>> for FloatRange
    where T: Into<f64>
{
    fn from(r: Range<T>) -> Self {
        FloatRange {
            start: r.start.into(),
            stop: r.end.into(),
            step: 1.0,
        }
    }
}

impl IntoIterator for FloatRange {
    type Item = f64;
    type IntoIter = FloatRangeIter;
    fn into_iter(self) -> Self::IntoIter {
        FloatRangeIter {
            current: self.start,
            stop: self.stop,
            step: self.step,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatRangeIter {
    current: f64,
    stop: f64,
    step: f64,
}

impl Iterator for FloatRangeIter {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.stop {
            None
        } else {
            let next = self.current;
            self.current += self.step;
            Some(next)
        }
    }
}


