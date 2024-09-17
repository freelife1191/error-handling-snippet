/// Pulling Results out of Options
///
/// 혼합된 오류 유형을 처리하는 가장 기본적인 방법은 오류 유형을 서로 삽입하는 것입니다.

use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_first() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];


        // The first doubled is Some(Ok(84))
        println!("The first doubled is {:?}", double_first(numbers));
        // The first doubled is None
        println!("The first doubled is {:?}", double_first(empty));
        // The first doubled is Some(Err(ParseIntError { kind: InvalidDigit }))
        // Error 1: the input vector is empty
        println!("The first doubled is {:?}", double_first(strings));
        // Error 2: the element doesn't parse to a number

    }
}