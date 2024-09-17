use std::{error, fmt};

#[allow(unused)]
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[allow(unused)]
type CustomResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[allow(unused)]
#[derive(Debug, Clone)]
struct CustomError;

impl error::Error for CustomError {}

#[allow(unused)]
#[derive(Debug, Clone)]
struct EmptyVec;

// 에러 메세지를 출력하기 위해 error::Error 트레이트를 구현한다
// into() 메소드를 사용하여 Box로 변환하고 Error 메세지를 주입한다
impl error::Error for EmptyVec {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}



#[allow(unused)]
fn double_first(vec: Vec<&str>) -> CustomResult<i32> {
    vec.first()
        // .ok_or_else(|| EmptyVec.into()) // Converts to Box
        .ok_or_else(|| CustomError.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                .map(|i| 2 * i)
        })
}

fn double_test(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().unwrap();
    // The first is tofu
    println!("The first is {}", first);
    let second = first.parse::<i32>().map(|i| 2 * i)?;
    Ok(2 * second)
}

fn print(result: CustomResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
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

    let strings2 = vec!["tofu", "93", "18"];
    // Error: invalid digit found in string
    print(double_test(strings.clone()));
}