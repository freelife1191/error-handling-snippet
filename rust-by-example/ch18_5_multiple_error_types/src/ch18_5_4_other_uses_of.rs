/// Other uses of ?
///
/// 이전 예에서 `parse` 호출에 대한 즉각적인 반응은 라이브러리 오류의 오류를 박스형 오류로 `매핑`하는 것입니다.

/*
.and_then(|s| s.parse::<i32>())
    .map_err(|e| e.into())
*/

/// 간단하고 흔한 작업이므로 생략할 수 있으면 편리할 것 같습니다.
/// 아쉽게도 `and_then`은 충분히 유연하지 않기 때문에 그렇게 할 수 없습니다. 그러나 대신 `?`를 사용할 수 있습니다.
///
/// `?`는 이전에 `unwrap` 또는 `return Err(err)`로 설명되었습니다.
/// 이것은 대부분 사실입니다.
/// 실제로는 `unwrap` 또는 `return Err(From::from(err))`을 의미합니다.
/// `From::from`은 서로 다른 유형 간의 변환 유틸리티이므로 오류가 반환 유형으로 변환 가능한 `?`를 사용하면 자동으로 변환된다는 의미입니다.
///
/// 여기서 `?`를 사용하여 이전 예제를 다시 작성합니다.
/// 결과적으로, 오류 유형에 대해 `From::from`이 구현되면 `map_err`은 사라질 것입니다:

use std::error;
use std::fmt;

// `Box<dyn error::Error>`를 사용하도록 별칭을 변경합니다.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

// 이전과 동일한 구조이지만 모든 `Results`를 연결하는 대신
// 그리고 `Options`에 따라 `?`를 사용하여 내부 값을 즉시 가져옵니다.
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/// 이제 실제로는 상당히 깨끗해졌습니다.
/// 원래 `panic`과 비교하면 `unwrap` 호출을 `?`로 바꾸는 것과 매우 유사합니다. 단, 반환 유형이 `Result`라는 점만 다릅니다.
/// 결과적으로 최상위 수준에서 구조를 해제해야 합니다.
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