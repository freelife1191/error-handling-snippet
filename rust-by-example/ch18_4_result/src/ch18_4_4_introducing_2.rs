/// The try! macro
///
/// `?`가 있기 전에는 `try!` 매크로를 사용하여 동일한 기능을 달성했습니다.
/// 이제 `?` 연산자가 권장되지만 이전 코드를 보면 여전히 `try!`를 찾을 수 있습니다.
/// 이전 예제와 동일한 `multiply` 함수는 `try!`를 사용하여 다음과 같습니다.

// 이 예제를 오류 없이 컴파일하고 실행하려면 `Cargo`를 사용하는 동안 값을 변경하세요.
// `Cargo.toml` 파일의 `[package]` 섹션에 있는 `edition` 필드를 "2015"로 변경합니다.

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = try!(first_number_str.parse::<i32>());
    let second_number = try!(second_number_str.parse::<i32>());

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
        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }
}