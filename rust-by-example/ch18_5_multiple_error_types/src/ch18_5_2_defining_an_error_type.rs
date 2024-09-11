/// Defining an error type
///
/// 때로는 단일 유형의 오류로 다양한 오류를 모두 가리도록 코드를 단순화합니다.
/// 이를 사용자 정의 오류와 함께 표시하겠습니다.
///
/// Rust를 사용하면 자체 오류 유형을 정의할 수 있습니다. 일반적으로 "양호" 오류 유형은 다음과 같습니다.
///
/// 동일한 유형의 다른 오류를 나타냅니다.
/// 사용자에게 좋은 오류 메시지를 표시합니다.
/// 다른 유형과 비교하기 쉽습니다.
///     - Good: `Err(EmptyVec)`
///     - Bad: `Err("Please use a vector with at least one element".to_owned())`
/// 오류에 대한 정보를 담을 수 있습니다.
///     - Good: `Err(BadChar(c, position))`
///     - Bad: `Err("+ cannot be used here".to_owned())`
/// 다른 오류와 잘 구성됩니다.

use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// 오류 유형을 정의합니다. 이는 오류 처리 사례에 맞게 사용자 정의될 수 있습니다.
// 이제 우리는 자체 오류를 작성하고 기본 오류를 연기할 수 있습니다.
// 구현하거나 그 사이에 뭔가를 합니다.
#[derive(Debug, Clone)]
struct DoubleError;

// 오류 생성은 오류가 표시되는 방식과 완전히 별개입니다.
// 표시 스타일로 인해 복잡한 로직이 복잡해지는 것에 대해 걱정할 필요가 없습니다.
//
// 오류에 대한 추가 정보는 저장하지 않습니다. 이는 우리가 말할 수 없다는 것을 의미합니다
// 해당 정보를 전달하기 위해 유형을 수정하지 않고 구문 분석에 실패한 문자열입니다.
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // 오류를 새로운 유형으로 변경합니다.
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // 여기에서도 새로운 오류 유형으로 업데이트됩니다.
                .map_err(|_| DoubleError)
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