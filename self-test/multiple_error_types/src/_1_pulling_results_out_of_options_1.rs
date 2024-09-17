use std::num::ParseIntError;

// unwrap 이 아니라 map 으로 처리하면 Option 으로 한번 감싸서 리턴하므로 panic 이 발생하지 않는다
fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

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
    println!("The first doubled is {:?}", double_first(strings));

}