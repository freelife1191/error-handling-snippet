/// Introducing ?
///
/// 때때로 우리는 `panic`의 가능성 없이 `unwrap`의 단순함을 원합니다.
/// 지금까지 `unwrap`은 우리가 정말로 원했던 것이 변수를 꺼내는 것이었음에도 불구하고 우리를 점점 더 깊게 중첩하도록 강요했습니다.
/// 이것이 바로 `?`의 목적입니다.
///
/// `Err`을 발견하면 취해야 할 두 가지 유효한 조치가 있습니다:
///
/// 1. 가능하면 피하려고 이미 결정한 `panic!`
/// 2. `Err`은 처리할 수 없다는 뜻이므로 `return`
///
/// `?`는 `Err`에 대한 `패닉` 대신 `반환`하는 `unwrap`과 정확히 동일합니다.
/// 결합자를 사용한 이전 예제를 어떻게 단순화할 수 있는지 살펴보겠습니다.

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        // n is 20
        print(multiply("10", "2"));
        // Error: invalid digit found in string
        print(multiply("t", "2"));
    }
}