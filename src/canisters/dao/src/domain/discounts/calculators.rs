use candid::Nat;
use num_traits::ToPrimitive;

pub struct ProportionCalculator {
    max_discount: f32,
}

/// Calculated the discount as 'price / staking_score'
impl ProportionCalculator {
    pub fn new(max_discount: f32) -> Self {
        Self { max_discount }
    }

    /// returns the discount value in percents, rounded up to 2nd digit after comma
    pub fn calculate_discount(&self, price: u128, staking_score: Nat) -> f32 {
        if price == 0 {
            return 0.0;
        }

        let score = staking_score.0.to_f32().unwrap_or(0.0);
        let price_f32 = price as f32;
        let raw = score / price_f32;

        let processed = Self::process_raw_result(raw);

        let result = if processed < self.max_discount {
            processed
        } else {
            self.max_discount
        };

        result
    }

    fn process_raw_result(raw: f32) -> f32 {
        let raw_percent = raw * 100.0;
        let ceiled = (raw_percent * 100.0).ceil();
        let result = ceiled / 100.0;

        result
    }
}

#[cfg(test)]
mod calc_test {
    use super::*;

    #[test]
    fn it_works() {
        let calc = ProportionCalculator::new(25.0);
        let res = calc.calculate_discount(15000, Nat::from(2000u128));
        assert_eq!(res, 13.34);
    }
}
