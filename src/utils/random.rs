use num_bigint::{BigUint, RandBigInt};
use num_traits::Num;
use crate::ranges::get_ranges;

pub fn gerar_hex_por_range(range_id: &str) -> Option<String> {
    let ranges = get_ranges();

    if let Some((min_hex, max_hex)) = ranges.get(range_id) {
        let min = BigUint::from_str_radix(min_hex, 16).ok()?;
        let max = BigUint::from_str_radix(max_hex, 16).ok()?;

        let mut rng = rand::thread_rng();
        let diff = &max - &min;
        let rand_offset = rng.gen_biguint_below(&diff);
        let result = &min + rand_offset;

        Some(format!("{:x}", result))
    } else {
        None
    }
}