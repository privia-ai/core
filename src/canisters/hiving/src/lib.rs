mod api;
mod services;

use candid::CandidType;
use serde::Serialize;
use abstractions::DiscountValue;



#[derive(Clone, Debug, Serialize, CandidType)]
struct DiscountQuote {
    discount_value: DiscountValue,
    price: u128
}