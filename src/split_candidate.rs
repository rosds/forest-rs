#[derive(PartialEq, Eq)]
pub enum SplitResult {
    Left,
    Right,
}

pub trait SplitCandidate {
    type InputData;

    ///
    /// A method to classify the InputData.
    ///
    fn classify(&self, data: &Self::InputData) -> SplitResult;

    ///
    /// A function that generates an SplitCandidate.
    ///
    fn generate() -> Self;
}
