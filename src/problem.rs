use core::slice;
use derive_new::new;
use std::{cell::RefCell, cmp::Ordering};

use rust_decimal::Decimal;

use crate::{Clock, Knapsack};

#[derive(new)]
pub struct Problem<'clocks> {
  clocks: &'clocks [Clock],
  max_weight: Decimal,
}

struct SolutionIter<'clocks> {
  stack: RefCell<Vec<(Knapsack, &'clocks [Clock])>>,
  visited: Vec<Knapsack>,
  max_weight: Decimal,
}

impl<'clocks> SolutionIter<'clocks> {
  fn find_valid_clock_index(clocks: &[Clock], max_weight: Decimal) -> Option<usize> {
    clocks
      .iter()
      .position(|clock| Knapsack::from_clocks(slice::from_ref(clock), max_weight).is_ok())
  }
}

impl<'clocks> Iterator for SolutionIter<'clocks> {
  type Item = Knapsack;

  fn next(&mut self) -> Option<Self::Item> {
    let mut bor_stack = self.stack.borrow_mut();

    if let Some((current_solution, remaining_clock)) = bor_stack.pop() {
      match remaining_clock {
        [] => {}
        [a, tail @ ..] => {
          if let Ok(updated) = current_solution.try_add_clock(*a) {
            bor_stack.push((updated.clone(), tail));
            bor_stack.push((
              Knapsack::from_clocks(slice::from_ref(a), current_solution.capacity).unwrap(),
              tail,
            ));
          }
          // push an option without current head, i.e. first from tail if any
          if let Some(index) = Self::find_valid_clock_index(tail, self.max_weight) {
            let next_head = Knapsack::from_clocks(&tail[index..=index], self.max_weight).unwrap();
            bor_stack.push((next_head, &tail[index + 1..]));
          }
        }
      }
      self.visited.push(current_solution.clone());
      return Some(current_solution.clone());
    }
    None
  }
}

impl Problem<'_> {
  fn iter(&self) -> SolutionIter<'_> {
    if let Some(index) = SolutionIter::find_valid_clock_index(self.clocks, self.max_weight) {
      let initial_solution =
        Knapsack::from_clocks(&self.clocks[index..=index], self.max_weight).unwrap();
      let remaining_clocks = &self.clocks[index + 1..];
      SolutionIter {
        stack: RefCell::new(vec![(initial_solution, remaining_clocks)]),
        visited: vec![],
        max_weight: self.max_weight,
      }
    } else {
      SolutionIter {
        stack: RefCell::new(vec![]),
        visited: vec![],
        max_weight: self.max_weight,
      }
    }
  }

  // used for testing purposes
  #[allow(dead_code)]
  fn get_all_combinations(&self) -> Vec<Knapsack> {
    self.iter().collect()
  }

  // Compare two Knapsacks by their cumulative monetary value
  fn cmp(first: &Knapsack, second: &Knapsack) -> Ordering {
    first
      .get_value()
      .partial_cmp(&second.get_value())
      .unwrap_or(Ordering::Equal)
  }

  pub fn get_best_knapsack(&self) -> Option<Knapsack> {
    self.iter().max_by(Self::cmp)
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use super::*;
  use expectest::prelude::*;
  use rstest::*;
  use rust_decimal_macros::dec;

  #[fixture]
  fn max_weight() -> Decimal {
    dec!(10.0)
  }

  #[fixture]
  fn three_clocks() -> Vec<Clock> {
    vec![
      Clock::new(dec!(5.0), dec!(6.0)),
      Clock::new(dec!(4.0), dec!(4.0)),
      Clock::new(dec!(6.0), dec!(7.0)),
    ]
  }

  #[rstest]
  fn empty_knapsack_for_zero_max_weight() {
    let problem = Problem {
      clocks: &[],
      max_weight: dec!(0.0),
    };

    let result = problem.get_all_combinations();

    expect!(result.iter()).to(be_empty());
  }

  #[rstest]
  fn empty_knapsack_for_positive_max_weight(max_weight: Decimal) {
    let problem = Problem {
      clocks: &[],
      max_weight,
    };

    let result = problem.get_all_combinations();

    expect!(result.iter()).to(be_empty());
  }

  #[rstest]
  fn knapsack_with_single_clock_below_max_weight(max_weight: Decimal) {
    let clock = Clock::new(dec!(5.0), dec!(20.0));
    let problem = Problem {
      clocks: &[clock],
      max_weight,
    };

    let result = problem.get_all_combinations();

    expect!(result.iter()).to(have_count(1));
  }

  #[rstest]
  fn knapsack_with_single_clock_exceeds_max_weight(max_weight: Decimal) {
    let clock = Clock::new(dec!(15.0), dec!(20.0));
    let problem = Problem {
      clocks: &[clock],
      max_weight,
    };

    let result = problem.get_all_combinations();

    expect!(result.iter()).to(be_empty());
  }

  #[rstest]
  fn knapsack_with_two_clocks_below_max_weight(max_weight: Decimal) {
    let clocks = vec![
      Clock::new(dec!(5.0), dec!(20.0)),
      Clock::new(dec!(4.0), dec!(10.0)),
    ];
    let problem = Problem {
      clocks: &clocks,
      max_weight,
    };

    let result = problem.get_all_combinations();

    let expected_clocks = vec![vec![clocks[0]], vec![clocks[1]], vec![clocks[0], clocks[1]]];
    let expected: Vec<Knapsack> = expected_clocks
      .into_iter()
      .filter_map(|clocks| Knapsack::from_clocks(&clocks, max_weight).ok())
      .collect();
    expect!(result).to(be_equal_to(expected));
  }

  #[rstest]
  fn knapsack_with_three_clocks_two_solutions(max_weight: Decimal, three_clocks: Vec<Clock>) {
    let problem = Problem {
      max_weight,
      clocks: &three_clocks,
    };

    let result = problem
      .get_all_combinations()
      .into_iter()
      .collect::<HashSet<Knapsack>>();

    let mut expected_clocks: HashSet<Vec<Clock>> = HashSet::new();
    expected_clocks.insert(vec![three_clocks[0]]);
    expected_clocks.insert(vec![three_clocks[1]]);
    expected_clocks.insert(vec![three_clocks[2]]);
    expected_clocks.insert(vec![three_clocks[0], three_clocks[1]]);
    expected_clocks.insert(vec![three_clocks[1], three_clocks[2]]);

    let expected = expected_clocks
      .into_iter()
      .filter_map(|clocks| Knapsack::from_clocks(&clocks, max_weight).ok())
      .collect::<HashSet<Knapsack>>();
    expect!(expected.is_subset(&result));
  }

  #[rstest]
  fn get_best_knapsack_for_two_solutions(max_weight: Decimal, three_clocks: Vec<Clock>) {
    let expected_best_solution = vec![three_clocks[1], three_clocks[2]];
    let problem = Problem {
      max_weight,
      clocks: &three_clocks,
    };

    problem
      .get_best_knapsack()
      .map(|actual_solution| assert_eq!(actual_solution.get_contents(), expected_best_solution))
      .unwrap_or_else(|| panic!("Expected a solution, but got None"));
  }

  #[rstest]
  fn get_best_knapsack_with_multiple_solutions(max_weight: Decimal) {
    let clocks = vec![
      Clock::new(dec!(2.0), dec!(5.0)),
      Clock::new(dec!(3.0), dec!(8.0)),
      Clock::new(dec!(5.0), dec!(12.0)),
      Clock::new(dec!(1.0), dec!(3.0)),
    ];

    let problem = Problem {
      max_weight,
      clocks: &clocks,
    };

    let expected_best_solution = vec![clocks[0], clocks[1], clocks[2]];

    problem
      .get_best_knapsack()
      .map(|actual_solution| assert_eq!(actual_solution.get_contents(), expected_best_solution))
      .unwrap_or_else(|| panic!("Expected a solution, but got None"));
  }
}
