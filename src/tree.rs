use super::parameters::Parameters;

pub struct Tree<S: SplitCandidate> {
    root: Box<TreeNode<S>>,
}

impl<S: SplitCandidate> Tree<S> {
    pub fn learn(params: &Parameters, data: &mut [(S::InputData, Label)]) -> Tree<S> {
        Tree {
            root: Box::new(build_tree::<S>(params, data)),
        }
    }

    pub fn classify(&self, data: &S::InputData) -> Label {
        self.root.classify(data).most_probable()
    }

    pub fn classify_confidence(&self, data: &S::InputData) -> &LabelDistribution {
        self.root.classify(data)
    }
}

///
/// private stuff here bellow
///
use super::labels::{Label, LabelDistribution};
use super::split_candidate::{SplitCandidate, SplitResult};

enum TreeNode<S: SplitCandidate> {
    SplitNode {
        s: S,
        left: Box<TreeNode<S>>,
        right: Box<TreeNode<S>>,
    },
    Leaf(LabelDistribution),
}

impl<S: SplitCandidate> TreeNode<S> {
    fn classify(&self, data: &S::InputData) -> &LabelDistribution {
        match self {
            TreeNode::Leaf(d) => d,
            TreeNode::SplitNode { s, left, right } => match s.classify(data) {
                SplitResult::Left => left.classify(data),
                SplitResult::Right => right.classify(data),
            },
        }
    }
}

fn get_label_distribution<S: SplitCandidate>(data: &[(S::InputData, Label)]) -> LabelDistribution {
    data.iter().map(|(_, l)| *l).collect()
}

fn entropy<S: SplitCandidate>(data: &[(S::InputData, Label)]) -> f64 {
    get_label_distribution::<S>(data).entropy()
}

fn information_gain<S: SplitCandidate>(split: usize, data: &[(S::InputData, Label)]) -> f64 {
    let total_size = data.len() as f64;
    let left_weight = data[..split].len() as f64 / total_size;
    let right_weight = data[split..].len() as f64 / total_size;

    let left_entropy = left_weight * entropy::<S>(&data[..split]);
    let right_entropy = right_weight * entropy::<S>(&data[split..]);
    let total_entropy = entropy::<S>(&data[..]);
    total_entropy - left_entropy - right_entropy
}

fn partition_in_place<S: SplitCandidate>(s: &S, data: &mut [(S::InputData, Label)]) -> usize {
    let mut split = 0usize;
    for i in 0..data.len() {
        let (x, _) = &data[i];
        if s.classify(&x) == SplitResult::Left {
            data.swap(split, i);
            split += 1;
        }
    }
    split
}

fn best_candidate<S: SplitCandidate>(params: &Parameters, data: &mut [(S::InputData, Label)]) -> S {
    let mut best_candidate = S::generate();
    let mut best_score = 0.0_f64;

    for _ in 0..params.candidates_to_try_per_node {
        let c = S::generate();
        let split = partition_in_place(&c, data);
        let score = information_gain::<S>(split, data);

        if score > best_score {
            best_candidate = c;
            best_score = score;
        }
    }

    best_candidate
}

fn build_tree<S: SplitCandidate>(
    params: &Parameters,
    data: &mut [(S::InputData, Label)],
) -> TreeNode<S> {
    // check stopping criteria
    let n = data.len();

    if n <= params.min_samples_per_leaf {
        TreeNode::<S>::Leaf(get_label_distribution::<S>(data))
    } else {
        let c = best_candidate::<S>(params, data);
        let split = partition_in_place::<S>(&c, data);

        let left_tree = Box::new(build_tree::<S>(params, &mut data[..split]));
        let right_tree = Box::new(build_tree::<S>(params, &mut data[split..]));

        TreeNode::<S>::SplitNode {
            s: c,
            left: left_tree,
            right: right_tree,
        }
    }
}
