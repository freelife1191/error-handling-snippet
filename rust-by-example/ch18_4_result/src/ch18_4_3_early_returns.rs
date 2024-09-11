/// Early returns
///
/// 이전 예에서는 연결자를 사용하여 오류를 명시적으로 처리했습니다.
/// 이 사례 분석을 처리하는 또 다른 방법은 '일치' 문과 조기 반환을 조합하여 사용하는 것입니다.
///
/// 즉, 함수 실행을 중지하고 오류가 발생하면 오류를 반환할 수 있습니다.
/// 일부의 경우 이 형식의 코드가 읽고 쓰기가 더 쉬울 수 있습니다.
/// 초기 반환을 사용하여 다시 작성된 이전 예제의 이 버전을 고려해보세요.

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/// 이 시점에서 우리는 결합자와 조기 반환을 사용하여 오류를 명시적으로 처리하는 방법을 배웠습니다.
/// 일반적으로 패닉을 피하고 싶지만 모든 오류를 명시적으로 처리하는 것은 번거롭습니다.
///
/// 다음 섹션에서는 `panic`을 유발하지 않고 `unwrap`만 하면 되는 경우를 위해 `?`를 소개하겠습니다.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }
}