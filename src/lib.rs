pub mod client;
pub mod problem;

use derive_new::new;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, new, Serialize, Deserialize)]
pub struct Clock {
  pub weight: Decimal,
  pub price: Decimal,
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GenericResult<T> = Result<T, GenericError>;

#[derive(Debug, Clone)]
pub enum AppError {
  KnapsackFullError,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Knapsack {
  contents: Vec<Clock>,
  capacity: Decimal,
}

impl Knapsack {
  pub fn from_clocks(clocks: &[Clock], max_weight: Decimal) -> Result<Self, AppError> {
    clocks
      .iter()
      .try_fold(Ok(Knapsack::empty(max_weight)), |result, &clock| {
        result.map(|knapsack| knapsack.try_add_clock(clock))
      })?
  }

  pub fn empty(max_weight: Decimal) -> Knapsack {
    Knapsack {
      contents: Vec::new(),
      capacity: max_weight,
    }
  }

  pub fn get_contents(&self) -> &[Clock] {
    &self.contents
  }

  pub fn try_add_clock(&self, clock: Clock) -> Result<Self, AppError> {
    if self.contents.iter().map(|c| c.weight).sum::<Decimal>() + clock.weight <= self.capacity {
      let mut new_solution = self.clone();
      new_solution.contents.push(clock);
      Ok(new_solution)
    } else {
      Err(AppError::KnapsackFullError)
    }
  }

  pub fn is_empty(&self) -> bool {
    self.get_contents().is_empty()
  }

  pub fn non_empty(&self) -> bool {
    !self.is_empty()
  }

  pub fn get_value(&self) -> Decimal {
    self.get_contents().iter().map(|c| c.price).sum()
  }
}

#[cfg(test)]
mod tests {
  use crate::{AppError, Clock, Knapsack};
  use expectest::prelude::*;
  use rstest::{fixture, rstest};
  use rust_decimal_macros::dec;
  use serde_json::{from_str, to_string};

  #[fixture]
  fn sample_clock() -> Clock {
    Clock::new(dec!(2.0), dec!(5.0))
  }

  #[rstest]
  fn empty_knapsack_should_contain_no_clocks() {
    let empty_knapsack = Knapsack::empty(dec!(5.0));
    assert!(empty_knapsack.contents.is_empty())
  }

  #[rstest]
  fn filled_knapsack_should_contain_all_clocks_passed_at_construction() -> Result<(), AppError> {
    let clocks = vec![
      Clock::new(dec!(0.5), dec!(19.99)),
      Clock::new(dec!(0.75), dec!(29.99)),
      Clock::new(dec!(0.9), dec!(39.99)),
    ];

    let filled_knapsack = Knapsack::from_clocks(&clocks, dec!(5.0))?;
    assert_eq!(&clocks, filled_knapsack.get_contents());
    Ok(())
  }

  #[rstest]
  fn one_should_be_able_to_add_clocks_to_contents_of_knapsack() -> Result<(), AppError> {
    let clock = Clock::new(dec!(4.45), dec!(2.29));
    let updated_knapsack = Knapsack::from_clocks(&[clock], dec!(5.0))?;

    let expected_contents = [clock];
    let actual_contents = updated_knapsack.get_contents();

    assert_eq!(&expected_contents[..], actual_contents);
    Ok(())
  }

  #[rstest]
  fn test_clock_serialization_deserialization(sample_clock: Clock) {
    // Serialize the Clock to JSON
    let serialized_json = to_string(&sample_clock).expect("Failed to serialize Clock to JSON");

    // Deserialize the JSON back to Clock
    let deserialized_clock: Clock =
      from_str(&serialized_json).expect("Failed to deserialize JSON to Clock");

    // Assert that the deserialized Clock is equal to the original
    expect!(deserialized_clock).to(be_equal_to(sample_clock));
  }
}
