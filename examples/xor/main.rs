extern crate random_forust;

use random_forust::parameters::Parameters;
use random_forust::split_candidate::{SplitCandidate, SplitResult};
use random_forust::tree::Tree;
use std::sync::atomic::{AtomicUsize, Ordering};

enum BitFilter {
    First,
    Second,
}

struct SimpleClassifier {
    bit: BitFilter,
}

impl SplitCandidate for SimpleClassifier {
    type InputData = (u8, u8);
    fn classify(&self, data: &Self::InputData) -> SplitResult {
        let predicate = match self.bit {
            BitFilter::First => data.0 == 0,
            BitFilter::Second => data.1 == 0,
        };
        if predicate {
            SplitResult::Left
        } else {
            SplitResult::Right
        }
    }

    fn generate() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        match COUNTER.fetch_add(1, Ordering::Relaxed) % 2 {
            0 => SimpleClassifier {
                bit: BitFilter::First,
            },
            1 => SimpleClassifier {
                bit: BitFilter::Second,
            },
            _ => SimpleClassifier {
                bit: BitFilter::First,
            },
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut dataset = vec![((1, 0), 1), ((0, 1), 1), ((0, 0), 0), ((1, 1), 0)];

    let param = Parameters {
        min_samples_per_leaf: 1,
        max_depth: 3,
        candidates_to_try_per_node: 4,
        ..Default::default()
    };

    let t = Tree::<SimpleClassifier>::learn(&param, &mut dataset);

    assert_eq!(t.classify(&(1, 0)), 1);
    assert_eq!(t.classify(&(0, 1)), 1);
    assert_eq!(t.classify(&(1, 1)), 0);
    assert_eq!(t.classify(&(0, 0)), 0);

    Ok(())
}
