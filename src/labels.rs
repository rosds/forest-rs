pub type Label = u8;

use std::collections::HashMap;

///
/// This represents a probability distribution over labels
///
pub struct LabelDistribution {
    dist: HashMap<Label, f64>,
}

impl LabelDistribution {
    pub fn most_probable(&self) -> Label {
        self.dist
            .iter()
            .fold((0u8, 0_f64), |a, (&l, &p)| if a.1 > p { a } else { (l, p) })
            .0
    }

    pub(crate) fn combine(mut self, other: &LabelDistribution) -> Self {
        for (k, v) in other.dist.iter() {
            *self.dist.entry(*k).or_insert(0.0_f64) += v;
        }
        self.normalize();
        self
    }

    pub(crate) fn entropy(&self) -> f64 {
        self.dist.values().map(|p| -p * p.log2()).sum()
    }

    pub(crate) fn new() -> Self {
        LabelDistribution {
            dist: HashMap::new(),
        }
    }

    fn normalize(&mut self) {
        let sum: f64 = self.dist.values().sum();
        for (_, val) in self.dist.iter_mut() {
            *val /= sum;
        }
    }
}

use std::iter::{FromIterator, IntoIterator};
impl FromIterator<Label> for LabelDistribution {
    fn from_iter<I: IntoIterator<Item = Label>>(iter: I) -> Self {
        let mut dist = LabelDistribution::new();
        for label in iter {
            *dist.dist.entry(label).or_insert(0.0_f64) += 1.0_f64;
        }
        dist.normalize();
        dist
    }
}
