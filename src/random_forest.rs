use super::labels::{Label, LabelDistribution};
use super::parameters::Parameters;
use super::split_candidate::SplitCandidate;
use super::tree::Tree;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct RandomForest<S: SplitCandidate> {
    forest: Vec<Tree<S>>,
}

impl<S: SplitCandidate> RandomForest<S> {
    pub fn learn(params: &Parameters, data: &mut [(S::InputData, Label)]) -> Self {
        let mut rng = thread_rng();

        let number_of_trees = params.number_of_trees;
        let samples_per_tree = params.samples_per_tree;

        let mut forest = Vec::new();
        for _ in 0..number_of_trees {
            data.shuffle(&mut rng);
            forest.push(Tree::<S>::learn(params, &mut data[0..samples_per_tree]));
        }

        RandomForest { forest: forest }
    }

    pub fn classify(&self, data: &S::InputData) -> Label {
        self.forest
            .iter()
            .map(|tree| tree.classify_confidence(&data))
            .fold(LabelDistribution::new(), LabelDistribution::combine)
            .most_probable()
    }
}
