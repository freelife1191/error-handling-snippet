/// 오류(`?`와 같은)에 대한 처리를 중지하고 싶을 때가 있지만 `Option`이 `None`일 때 계속 진행합니다.
/// `transpose` 함수는 `Result`와 `Option`을 교환하는 데 유용합니다.

use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_first() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        println!("The first doubled is {:?}", double_first(numbers));
        println!("The first doubled is {:?}", double_first(empty));
        println!("The first doubled is {:?}", double_first(strings));
    }
}