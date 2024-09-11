/// Boxing errors
///
/// 원래 오류를 유지하면서 간단한 코드를 작성하는 방법은 오류를 '박스'로 묶는 것입니다.
/// 단점은 기본 오류 유형이 런타임에만 알려지고 `정적으로 결정`되지 않는다는 것입니다.
///
/// stdlib는 `Box`가 `Error` 특성을 구현하는 모든 유형에서 `From`을 통해 특성 개체 `Box<Error>`로의 변환을 구현하도록 하여 오류를 박싱하는 데 도움이 됩니다.

use std::error;
use std::fmt;

// Change the alias to use `Box<dyn error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_first() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}