mod ch18_5_2_defining_an_error_type;
mod ch18_5_3_boxing_errors;
mod ch18_5_4_other_uses_of;
mod ch18_5_5_wrapping_errors;
mod ch18_5_1_pulling_results_out_of_options_2;
mod ch18_5_1_pulling_results_out_of_options_1;

/// Multiple error types
///
/// 이전 예제는 항상 매우 편리했습니다.
/// `Result`는 다른 `Result`와 상호 작용하고 `Option`은 다른 `Option`과 상호 작용합니다.
///
/// 때로는 `Option`이 `Result`와 상호작용해야 하거나 `Result<T, Error1>`이 `Result<T, Error2>`와 상호작용해야 하는 경우도 있습니다.
/// 이러한 경우 다양한 오류 유형을 구성 가능하고 쉽게 상호 작용할 수 있는 방식으로 관리하고 싶습니다.
///
/// 다음 코드에서 `unwrap`의 두 인스턴스는 서로 다른 오류 유형을 생성합니다.
/// `Vec::first`는 `Option`을 반환하고 `parse::<i32>`는 `Result<i32, ParseIntError>`를 반환합니다.

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

/// 다음 섹션에서는 이러한 종류의 문제를 처리하기 위한 몇 가지 전략을 살펴보겠습니다.
fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    // The first doubled is 84
    println!("The first doubled is {}", double_first(numbers));
    // called `Option::unwrap()` on a `None` value
    // println!("The first doubled is {}", double_first(empty));
    println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty
    // thread 'main' panicked at src/main.rs:10:30:
    // called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
    println!("The first doubled is {}", double_first(strings));
    // Error 2: the element doesn't parse to a number
}