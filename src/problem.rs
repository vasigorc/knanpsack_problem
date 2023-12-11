use std::collections::HashSet;

use crate::{Clock, Knapsack};

struct Problem {
    max_weight: f32,
    knapsack: Knapsack,
}

impl Problem {
    fn get_all_combinations(
        &self,
        clocks: &[Clock],
        current_solution: Vec<Clock>,
        mut accumulator: HashSet<Vec<Clock>>,
    ) -> HashSet<Vec<Clock>> {
        match clocks {
            [] => accumulator,
            [a] => {
                let mut updated = current_solution.clone();
                updated.push(*a);

                if updated.iter().map(|x| x.weight_as_f32()).sum::<f32>() <= self.max_weight {
                    accumulator.insert(updated);
                }
                accumulator
            }
            [a, tail @ ..] => {
                let mut updated = current_solution.clone();
                updated.push(*a);

                if updated.iter().map(|x| x.weight_as_f32()).sum::<f32>() <= self.max_weight {
                    accumulator.insert(updated.clone());
                    accumulator = self.get_all_combinations(tail, updated.clone(), accumulator);
                    accumulator.extend(self.get_all_combinations(
                        &updated,
                        current_solution.clone(),
                        accumulator.clone(),
                    ));
                }
                accumulator.extend(self.get_all_combinations(
                    tail,
                    current_solution,
                    accumulator.clone(),
                ));

                accumulator
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;
    use rstest::*;

    #[rstest]
    fn empty_knapsack_for_zero_max_weight() {
        let problem = Problem {
            max_weight: 0.0,
            knapsack: Knapsack::empty(),
        };

        let result =
            problem.get_all_combinations(&problem.knapsack.contents, Vec::new(), HashSet::new());

        expect!(result.iter()).to(be_empty());
    }

    #[rstest]
    fn empty_knapsack_for_positive_max_weight() {
        let problem = Problem {
            max_weight: 10.0,
            knapsack: Knapsack::empty(),
        };

        let result =
            problem.get_all_combinations(&problem.knapsack.contents, Vec::new(), HashSet::new());

        expect!(result.iter()).to(be_empty());
    }

    #[rstest]
    fn knapsack_with_single_clock_below_max_weight() {
        let clock = Clock::from_f32(5.0, 20.0);
        let knapsack = clock
            .map(|clock| Knapsack::from_clocks(&[clock]))
            .unwrap_or_default();
        let problem = Problem {
            max_weight: 10.0,
            knapsack,
        };

        let result =
            problem.get_all_combinations(&problem.knapsack.contents, Vec::new(), HashSet::new());

        expect!(result.iter()).to(have_count(1));
    }

    #[rstest]
    fn knapsack_with_single_clock_exceeds_max_weight() {
        let clock = Clock::from_f32(15.0, 20.0);
        let knapsack = clock
            .map(|clock| Knapsack::from_clocks(&[clock]))
            .unwrap_or_default();
        let problem = Problem {
            max_weight: 10.0,
            knapsack,
        };

        let result =
            problem.get_all_combinations(&problem.knapsack.contents, Vec::new(), HashSet::new());

        expect!(result.iter()).to(be_empty());
    }

    #[rstest]
    fn knapsack_with_two_clocks_below_max_weight() {
        let maybe_clocks = vec![Clock::from_f32(5.0, 20.0), Clock::from_f32(4.0, 10.0)];
        let clocks: Vec<_> = maybe_clocks.into_iter().map(Option::unwrap).collect();
        let problem = Problem {
            max_weight: 10.0,
            knapsack: Knapsack::from_clocks(&clocks),
        };

        let result =
            problem.get_all_combinations(&problem.knapsack.contents, Vec::new(), HashSet::new());

        let mut expected = HashSet::new();
        expected.insert(vec![clocks[0]]);
        expected.insert(vec![clocks[1]]);
        expected.insert(vec![clocks[0], clocks[1]]);
        expect!(result).to(be_equal_to(expected));
    }
}
