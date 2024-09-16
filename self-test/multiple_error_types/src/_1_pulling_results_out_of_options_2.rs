use std::num::ParseIntError;

// Option 의 transpose 함수를 사용하면 Result 를 반환하면서 값은 Option 으로 반환한다
// Err 은 Result로 반환되고 Ok 는 Option 으로 반환된다
fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.transpose()
}

#[test]
fn test_double_first() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));
}