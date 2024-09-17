#[allow(unused)]
mod _1_map_for_result_1;
mod _1_map_for_result_2;
mod _2_aliases_for_result;
mod _3_early_returns;
mod _4_introducing_1;

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    println!("first_number: {}, second_number: {}", first_number, second_number);
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    // double is 20
    println!("double is {}", twenty);
    assert_eq!(twenty, 20);
    // thread 'main' panicked at src/main.rs:9:56:
    // called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
    let tt = multiply("t", "2");
    assert_eq!(tt, 20);
}