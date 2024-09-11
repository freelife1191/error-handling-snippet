/// aliases for Result
///
/// 특정 `Result` 유형을 여러 번 재사용하려는 경우는 어떻습니까? Rust를 사용하면 '별칭'을 만들 수 있다는 점을 기억하세요.
/// 편리하게도 문제의 특정 `결과`에 대해 하나를 정의할 수 있습니다.
///
/// 모듈 수준에서 별칭을 만드는 것이 특히 도움이 될 수 있습니다.
/// 특정 모듈에서 발견된 오류는 동일한 `Err` 유형을 갖는 경우가 많으므로 단일 별칭으로 관련된 모든 `Results`를 간결하게 정의할 수 있습니다.
/// 이것은 `std` 라이브러리가 `io::Result`를 제공할 정도로 매우 유용합니다!
///
/// 다음은 구문을 보여주는 간단한 예입니다.

use std::num::ParseIntError;

// 오류 유형 `ParseIntError`를 사용하여 `Result`에 대한 일반 별칭을 정의합니다.
type AliasedResult<T> = Result<T, ParseIntError>;

// 위의 별칭을 사용하여 특정 `Result` 유형을 참조합니다.
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// 여기서 별칭을 사용하면 공간을 절약할 수 있습니다.
fn print(result: AliasedResult<i32>) {
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
        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }
}