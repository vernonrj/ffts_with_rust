use std::ops::{AddAssign, Range, RangeBounds, Bound};
use num_traits::Num;

/**
 * A range with a step
 */
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct StepRange<Idx> {
    pub start: Idx,
    pub end: Idx,
    pub step: Idx,
}

impl<Idx> RangeBounds<Idx> for StepRange<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Excluded(&self.end)
    }
}

impl<Idx> StepRange<Idx> {
    /// New range with a new step
    pub fn with_step(self, new_step: Idx) -> Self {
        StepRange {
            step: new_step,
            ..self
        }
    }
}

impl<Idx> StepRange<Idx>
    where Idx: Num + Clone + From<u32>,
{
    /// New range with an adjusted step to contain `num_points`
    pub fn with_num_points(self, new_points: usize) -> Self {
        let new_step = (self.end.clone() - self.start.clone()) / (new_points as u32).into();
        StepRange {
            step: new_step,
            ..self
        }
    }
}

impl<Idx, T> From<Range<T>> for StepRange<Idx>
    where T: Into<Idx>,
          Idx: From<u32>
{
    fn from(r: Range<T>) -> Self {
        StepRange {
            start: r.start.into(),
            end: r.end.into(),
            step: 1u32.into(),
        }
    }
}

impl<Idx> IntoIterator for StepRange<Idx>
    where Idx: Num + Clone + PartialOrd + AddAssign,
{
    type Item = Idx;
    type IntoIter = StepRangeIter<Idx>;
    fn into_iter(self) -> Self::IntoIter {
        StepRangeIter {
            current: self.start,
            end: self.end,
            step: self.step,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepRangeIter<Idx> {
    current: Idx,
    end: Idx,
    step: Idx,
}

impl<Idx> Iterator for StepRangeIter<Idx>
    where Idx: Num + Clone + PartialOrd + AddAssign,
{
    type Item = Idx;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let next = self.current.clone();
            self.current += self.step.clone();
            Some(next)
        }
    }
}


