fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}


// filter_map()을 사용해서 None 결과는 필터링 해서 있는 값만 리턴
// Results: [93, 18]
#[test]
fn test_filter_map() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);
}


// filter_map을 통해 error를 필터링 하면서 에러를 map_err 로 전달하고
// map_err() 를 통해 error 에 대한 별도의 처리가 가능함
// Numbers: [42, 93, 18]
// Errors: [ParseIntError { kind: InvalidDigit }, ParseIntError { kind: PosOverflow }]
#[test]
fn test_map_err_and_filter_map() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}


// collect 의 FromIterator 를 구현하여 실패가 있으면 전체 작업을 실패처리 함
// Results: Err(ParseIntError { kind: InvalidDigit })
#[test]
fn test_collect() {
    let strings = vec!["tofu", "93", "18"];
    // fn collect<B: FromIterator<Self::Item>>(self) -> B
    // 아래와 같이 타입을 명시하지않으면 let numbers: Vec<_> = strings 기본으로 이렇게 설정되어 에러가 발생함
    // 타입을 아래와 같이 명시하여 Result<Vec<_>, _> 로 Wrapping 하여 에러를 처리함
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}


// partition을 사용하면 각각의 변수에 Result 타입의 성공값과 실패값을 모두 수집할 수 있음
// Numbers: [Ok(93), Ok(18)]
// Errors: [Err(ParseIntError { kind: InvalidDigit })]
#[test]
fn test_partition() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

// partition 의 결과값은 Result 타입이여서 into_iter와 map 을 사용해여 순회하면서 unwrap 처리
// Numbers: [93, 18]
// Errors: [ParseIntError { kind: InvalidDigit }]
#[test]
fn test_partition_boilerplate() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}