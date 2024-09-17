/// Wrapping errors
///
/// 박싱 오류에 대한 대안은 오류를 자신만의 오류 유형으로 래핑하는 것입니다.

use std::error;
use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // 오류에 대해서는 구문 분석 오류 구현을 따르겠습니다.
    // 추가 정보를 제공하려면 유형에 더 많은 데이터를 추가해야 합니다.
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            // 래핑된 오류에는 추가 정보가 포함되어 있으며 사용 가능합니다.
            // source() 메소드를 통해.
            DoubleError::Parse(..) =>
                write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            // 원인은 기본 구현 오류 유형입니다. 암시적으로
            // 특성 객체 `&error::Error`로 캐스팅합니다. 이것은 작동합니다.
            // 기본 유형은 이미 `Error` 특성을 구현합니다.
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

// `ParseIntError`에서 `DoubleError`로의 변환을 구현합니다.
// `ParseIntError`가 발생한 경우 `?`에 의해 자동으로 호출됩니다.
// `DoubleError`로 변환되어야 합니다.
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    // 여기서는 `DoubleError`를 생성하기 위해 위에서 정의한 `From`의 `ParseIntError` 구현을 암시적으로 사용합니다.
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        },
    }
}

/// 이는 오류 처리를 위한 상용구를 조금 더 추가하며 일부 애플리케이션에서는 필요하지 않을 수도 있습니다.
/// 상용구를 처리해 줄 수 있는 라이브러리가 있습니다.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_first() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        // The first doubled is 84
        print(double_first(numbers));
        // Error: please use a vector with at least one element
        print(double_first(empty));
        // ParseIntError Convert: ParseIntError { kind: InvalidDigit }
        // Error: the provided string could not be parsed as int
        //   Caused by: invalid digit found in string
        print(double_first(strings));
    }
}