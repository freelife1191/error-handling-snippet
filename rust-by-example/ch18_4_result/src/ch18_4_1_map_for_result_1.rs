/// map for Result
///
/// 이전 예제의 `multiply`에서 패닉 상태가 된다고 해서 강력한 코드가 되는 것은 아닙니다.
/// 일반적으로 우리는 오류에 응답하는 올바른 방법이 무엇인지 결정할 수 있도록 호출자에게 오류를 반환하려고 합니다.
///
/// 먼저 우리가 다루고 있는 오류 유형이 무엇인지 알아야 합니다.
/// `Err` 유형을 결정하기 위해 `i32`에 대한 `FromStr` 특성으로 구현된 `parse()`를 살펴봅니다.
/// 결과적으로 `Err` 유형은 `ParseIntError`로 지정됩니다.
///
/// 아래 예에서 간단한 `match` 문은 전체적으로 더 성가신 코드로 이어집니다.

use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number)  => {
            match second_number_str.parse::<i32>() {
                Ok(second_number)  => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
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
        // 이것은 여전히 합리적인 답변을 제시합니다.
        let twenty = multiply("10", "2");
        // n is 20
        print(twenty);

        // 이제 다음은 훨씬 더 유용한 오류 메시지를 제공합니다.
        let tt = multiply("t", "2");
        // Error: invalid digit found in string
        print(tt);
    }
}