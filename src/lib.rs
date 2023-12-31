use rstest::rstest;
use rust_decimal::{prelude::FromPrimitive, Decimal};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Clock {
    pub weight: Decimal,
    pub price: Decimal,
}

impl Clock {
    pub fn from_f32(weight: f32, price: f32) -> Option<Clock> {
        let dec_weight = Decimal::from_f32(weight)?;
        let dec_price = Decimal::from_f32(price)?;
        Some(Clock {
            weight: dec_weight,
            price: dec_price,
        })
    }
}

pub struct Knapsack {
    contents: Vec<Clock>,
}

impl Knapsack {
    pub fn from_clocks(clocks: &[Clock]) -> Knapsack {
        Knapsack {
            contents: clocks.iter().cloned().collect(),
        }
    }

    pub fn empty() -> Knapsack {
        Knapsack {
            contents: Vec::new(),
        }
    }

    pub fn get_contents(&self) -> &[Clock] {
        &self.contents
    }

    pub fn add_clock(&self, clock: Clock) -> Knapsack {
        let mut new_contents = self.contents.clone();
        new_contents.push(clock);

        Knapsack {
            contents: new_contents,
        }
    }
}

impl Default for Knapsack {
    fn default() -> Self {
        Knapsack::empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::Knapsack;

    use super::*;

    #[rstest]
    fn empty_knapsack_should_contain_no_clocks() {
        let empty_knapsack = Knapsack::empty();
        assert!(empty_knapsack.contents.is_empty())
    }

    #[rstest]
    fn filled_knapsack_should_contain_all_clocks_passed_at_construction() {
        let clocks = vec![
            Clock::from_f32(0.5, 19.99),
            Clock::from_f32(0.75, 29.99),
            Clock::from_f32(0.9, 39.99),
        ];

        let valid_clocks: Vec<_> = clocks.into_iter().map(Option::unwrap).collect();

        let filled_knapsack = Knapsack::from_clocks(&valid_clocks);
        assert_eq!(&valid_clocks, filled_knapsack.get_contents());
    }

    #[rstest]
    fn one_should_be_able_to_add_clocks_to_contents_of_knapsack() {
        let clock = Clock::from_f32(4.45, 2.29);
        let updated_knapsack = clock
            .map(|clock| Knapsack::from_clocks(&[clock]))
            .unwrap_or_default();

        let expected_contents = clock.map_or_else(Vec::new, |clock| vec![clock]);
        let actual_contents = updated_knapsack.get_contents();

        assert_eq!(&expected_contents[..], actual_contents);
    }
}
