mod _1_pulling_results_out_of_options_1;
mod _1_pulling_results_out_of_options_2;
mod _2_defining_an_error_type;
mod _3_boxing_errors;
mod _4_other_uses_of;
mod _5_wrapping_errors;

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1 (Option)
    2 * first.parse::<i32>().unwrap() // Generate error 2 (Result)
}

#[should_panic]
fn main() {
    let numbers = vec!["42", "93", "18"];
    // let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    // The first doubled is 84
    println!("The first doubled is {}", double_first(numbers));
    // 빈 Vec 에 대한 unwrap은 패닉을 일으킵니다
    // thread 'main' panicked at src/main.rs:9:29:
    // called `Option::unwrap()` on a `None` value
    // println!("The first doubled is {}", double_first(empty));
    // parse에 대한 unwrap은 패닉을 일으킵니다
    // thread 'main' panicked at src/main.rs:10:30:
    // called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
    println!("The first doubled is {}", double_first(strings));
}
