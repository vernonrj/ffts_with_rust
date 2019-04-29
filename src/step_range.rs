use std::ops::{Range, RangeBounds, Bound};

/**
 * A range with a step
 */
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct StepRange {
    pub start: f64,
    pub end: f64,
    pub step: f64,
}

impl RangeBounds<f64> for StepRange {
    fn start_bound(&self) -> Bound<&f64> {
        Bound::Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&f64> {
        Bound::Excluded(&self.end)
    }
}

impl StepRange {
    /// New range with a new step
    pub fn with_step(self, new_step: f64) -> Self {
        StepRange {
            step: new_step,
            ..self
        }
    }
    /// New range with an adjusted step to contain `num_points`
    pub fn with_num_points(self, new_points: usize) -> Self {
        let new_step = (self.end - self.start) / new_points as f64;
        StepRange {
            step: new_step,
            ..self
        }
    }
}

impl<T> From<Range<T>> for StepRange
    where T: Into<f64>
{
    fn from(r: Range<T>) -> Self {
        StepRange {
            start: r.start.into(),
            end: r.end.into(),
            step: 1.0,
        }
    }
}

impl IntoIterator for StepRange {
    type Item = f64;
    type IntoIter = StepRangeIter;
    fn into_iter(self) -> Self::IntoIter {
        StepRangeIter {
            current: self.start,
            end: self.end,
            step: self.step,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepRangeIter {
    current: f64,
    end: f64,
    step: f64,
}

impl Iterator for StepRangeIter {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let next = self.current;
            self.current += self.step;
            Some(next)
        }
    }
}


