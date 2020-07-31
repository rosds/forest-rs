# random-forust

A simple Decision Tree and Random Forest implementation in rust.


## How to use it?

To use the library you need to provide a type that implements the
`SplitCandidate` trait. 

```rust

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

```

The associated type `InputData` is the input type for your problem; the type of
data that you want to classify. Additionally you have to provide a way to
generate different instances of this type.

In theory, you only need some classifier that is able to split the data in a
way that is better than random.

Then you can build a decision tree by providing parameters telling the
algorithm when to stop growing the tree and the input labeled data.


```rust

    struct WeatherConditions {
      is_rainy: bool,
      is_sunny: bool,
      humidity_level: f64,
      ...
    }
    
    // some data to learn from
    let mut tennis_games = vec![
      (WeatherConditions{ ... }, result),
      (WeatherConditions{ ... }, result),
      (WeatherConditions{ ... }, result),
      ...
    ];

    let params = Parameters {
        min_samples_per_leaf: 1,
        candidates_to_try_per_node: 4,
        ..Default::default()
    };

    let tree = Tree::<MyClassifierType>::learn(&params, &mut tennis_games);

    println!("predicted result: {}", tree.classify(&unseen_weather_conditions));

```
