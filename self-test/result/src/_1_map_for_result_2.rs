use std::num::ParseIntError;

// and_then 을 사용하면 map 을 사용한 것과 같은 효과를 낼 수 있다
// and_then 은 Result 가 Ok 일 때만 실행되고 Err 일 때는 바로 Err 를 반환한다
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn result_test(first_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>()
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
#[allow(unused)]
fn test_multiply() {
    let twenty = multiply("10", "2");
    // n is 20
    print(twenty);

    let tt = multiply("t", "2");
    // error Err(ParseIntError { kind: InvalidDigit })
    println!("error {:?}", tt);
    // print(tt);

    // result Err(ParseIntError { kind: InvalidDigit })
    print!("result {:?}", result_test("t"));
}