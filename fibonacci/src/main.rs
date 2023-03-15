const GOLDEN_RATIO: f64 = 1.618034;

fn main() {
    let nth = 6;

    println!(
        "{} is the {} fibonacci number.",
        get_fibonacci(nth),
        (nth + 1)
    )
}

fn get_fibonacci(nth: u32) -> u32 {
    div_by_sqrt_of_5(gr_to_pow_of(nth as i32) - one_minus_gr_to_pow_of(nth as i32))
}

fn gr_to_pow_of(nth: i32) -> f64 {
    GOLDEN_RATIO.powi(nth)
}

fn one_minus_gr_to_pow_of(nth: i32) -> f64 {
    (1.0 - GOLDEN_RATIO).powi(nth)
}

fn div_by_sqrt_of_5(num: f64) -> u32 {
    (num / (5 as f64).sqrt()).round() as u32
}
