use std::num::ParseIntError;

// ? 는 Err 발생시 panic 이 아니라 Err 를 return 한다
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    println!("first_number: {}", first_number);
    let second_number = second_number_str.parse::<i32>()?;
    println!("second_number: {}", second_number);
    Ok(first_number * second_number)
}

// Result 에서 match 를 사용하여 error 를 출력하면 에러 메시지를 출력할 수 있다
fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn test_multiply() {
    // n is 20
    print(multiply("10", "2"));
    // Error: invalid digit found in string
    print(multiply("t", "2"));
    // Err(ParseIntError { kind: InvalidDigit })
    println!("{:?}", multiply("t", "2"))
}