mod ch18_4_1_map_for_result_1;
mod ch18_4_2_aliases_for_result;
mod ch18_4_3_early_returns;
mod ch18_4_4_introducing_1;

/// Result
/// `Result`는 가능한 부재 대신 가능한 오류를 설명하는 `Option` 유형의 더욱 풍부한 버전입니다.
///
/// 즉, `Result<T, E>`는 두 가지 결과 중 하나를 가질 수 있습니다:
///
/// `Ok(T)`: 요소 `T`를 찾았습니다.
/// `Err(E)`: 요소 `E`에서 오류가 발견되었습니다.
///
/// 관례적으로 예상되는 결과는 `Ok`이고 예상치 못한 결과는 `Err`입니다.
///
/// `Option`과 마찬가지로 `Result`에도 이와 관련된 많은 메서드가 있습니다. 예를 들어 `unwrap()`은 `T` 요소 또는 `panic` 요소를 생성합니다.
/// 사례 처리를 위해 `Result`와 `Option` 사이에 겹치는 결합자가 많이 있습니다.
///
/// Rust로 작업하다 보면 `parse()` 메소드와 같이 `Result` 유형을 반환하는 메소드를 접하게 될 것입니다.
/// 문자열을 다른 유형으로 구문 분석하는 것이 항상 가능하지는 않을 수 있으므로 `parse()`는 실패 가능성을 나타내는 `Result`를 반환합니다.
///
/// 문자열 `parse()`에 성공할 때와 실패할 때 무슨 일이 일어나는지 봅시다:
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

/// 실패한 경우 `parse()`는 `panic`에 대한 `unwrap()`에 대한 오류를 남깁니다.
/// 또한 `panic`은 프로그램을 종료하고 불쾌한 오류 메시지를 제공합니다.
///
/// 오류 메시지의 품질을 향상하려면 반환 유형을 보다 구체적으로 지정하고 오류를 명시적으로 처리하는 것을 고려해야 합니다.
fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}

/// 결과를 메인에서 사용하기
///
/// `Result` 유형은 명시적으로 지정된 경우 `main` 함수의 반환 유형이 될 수도 있습니다.
/// 일반적으로 `main` 함수의 형식은 다음과 같습니다.
/*
fn main() {
    println!("Hello World!");
}
*/

/// 하지만 `main`은 `Result` 반환 유형도 가질 수 있습니다.
/// `main` 함수 내에서 오류가 발생하면 오류 코드를 반환하고 오류의 디버그 표현을 인쇄합니다(`Debug` 특성 사용).
/// 다음 예는 그러한 시나리오를 보여주고 `다음 섹션`에서 다루는 측면을 다룹니다.
#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    #[test]
    fn test_main_result() -> Result<(), ParseIntError> {
        let number_str = "10";
        let number = match number_str.parse::<i32>() {
            Ok(number)  => number,
            Err(e) => return Err(e),
        };
        println!("{}", number);
        Ok(())
    }
}