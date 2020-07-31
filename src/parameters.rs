#[derive(Default)]
pub struct Parameters {
    pub max_depth: usize,
    pub min_samples_per_leaf: usize,
    pub candidates_to_try_per_node: usize,
    pub samples_per_tree: usize,
    pub number_of_trees: usize,
}
