/// 다행스럽게도 `Option`의 `map`, `and_then` 및 기타 많은 조합자도 `Result`에 대해 구현됩니다.
/// `Result`에는 전체 목록이 포함되어 있습니다.

use std::num::ParseIntError;

// `Option`과 마찬가지로 `map()`과 같은 결합자를 사용할 수 있습니다.
// 이 함수는 위의 함수와 동일하며 다음과 같습니다.
// str에서 두 값을 모두 구문 분석할 수 있으면 곱하고, 그렇지 않으면 오류를 전달합니다.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
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
        print(twenty);

        // 이제 다음은 훨씬 더 유용한 오류 메시지를 제공합니다.
        let tt = multiply("t", "2");
        print(tt);
    }
}