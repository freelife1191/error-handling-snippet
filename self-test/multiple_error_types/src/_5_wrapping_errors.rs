use std::{error, fmt};
use std::error::Error;
use std::num::ParseIntError;

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

// ParseIntError 가 발생하면 DoubleError로 변환되도록 처리
// 구현하지 않으면 DoubleError 가 ParseIntError 타입의 에러를 처리할 수 없어서 에러가 발생함
// 추가적인 에러 처리를 위해서는 모든 타입의 에러 구현체를 구현 해주어야 함
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        println!("ParseIntError Convert: {:?}", err);
        DoubleError::Parse(err)
    }
}


// DoubleError Result를 반환하므로 ParseIntError 가 발생하면 DoubleError로 자동으로 변환됨
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    // 여기서는 `DoubleError`를 생성하기 위해 위에서 정의한 `From`의 `ParseIntError` 구현을 암시적으로 사용합니다.
    let parsed = first.parse::<i32>()?;
    // let parsed = first.parse::<i32>().map_or_else(|e| Err(DoubleError::Parse(e)), Ok)?;

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            // 위에서 Error를 오버라이딩 하여 구현한 source() 메소드를 통해서 원인이 출력 가능함
            // Error 내부에 source 메소드가 포함되어 있음
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        },
    }
}

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
    print(double_first(strings.clone()));
}