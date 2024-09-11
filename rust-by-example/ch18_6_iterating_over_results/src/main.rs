/// Iterating over Results
///
/// `Iter::map` 작업이 실패할 수 있습니다. 예를 들면 다음과 같습니다.
/// 이를 처리하기 위한 전략을 단계별로 살펴보겠습니다.
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    /// `filter_map()`을 사용하여 실패한 항목을 무시합니다.
    ///
    /// `filter_map`은 함수를 호출하고 `None`인 결과를 필터링합니다.
    #[test]
    fn test_filter_map() {
        let strings = vec!["tofu", "93", "18"];
        let numbers: Vec<_> = strings
            .into_iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        println!("Results: {:?}", numbers);
    }

    /// `map_err()` 및 `filter_map()`을 사용하여 실패한 항목을 수집합니다.
    ///
    /// `map_err`은 오류가 있는 함수를 호출하므로 이를 이전 `filter_map` 솔루션에 추가하여 반복하는 동안 옆에 저장할 수 있습니다.
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

    /// `collect()`를 사용하여 전체 작업을 실패합니다.
    ///
    /// `Result`는 결과 벡터(`Vec<Result<T, E>>`)가 벡터(`Result<Vec<T>, E>`)를 사용하여 결과로 바뀔 수 있도록 `FromIterator`를 구현합니다.
    /// `Result::Err`이 발견되면 반복이 종료됩니다.
    ///
    /// 이와 동일한 기술을 `Option`과 함께 사용할 수 있습니다.
    #[test]
    fn test_collect() {
        let strings = vec!["tofu", "93", "18"];
        let numbers: Result<Vec<_>, _> = strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            .collect();
        println!("Results: {:?}", numbers);
    }

    /// `partition()`을 사용하여 유효한 값과 실패를 모두 수집합니다.
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

    /// 결과를 보면 모든 것이 여전히 `Result`에 포함되어 있음을 알 수 있습니다.
    /// 이를 위해서는 좀 더 `boilerplate`가 필요합니다.
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
}