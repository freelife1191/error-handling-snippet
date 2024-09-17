use std::{error, fmt};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}


impl error::Error for EmptyVec {}

// ok_or()와 parse() 호출에 대한 즉각적인 반응은 라이브러리 오류의 오류를 박스형 오류로 매핑하는 것입니다.
// Option에 따라 ? 를 사용하여 내부 값을 즉시 가져온다
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

#[test]
fn test_double_first() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    // The first doubled is 84
    print(double_first(numbers));
    // Error: invalid first item to double
    print(double_first(empty));
    // Error: invalid digit found in string
    print(double_first(strings.clone()));
}