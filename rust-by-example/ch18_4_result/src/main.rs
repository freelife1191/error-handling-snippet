mod ch18_4_1_map_for_result;
mod ch18_4_2_aliases_for_result;
mod ch18_4_3_early_returns;
mod ch18_4_4_introducing;
mod using_result;

/// 결과는 부재 가능성 대신 가능한 오류를 설명하는 더 풍부한 옵션 유형 버전입니다.
///
/// 즉, Result<T, E>는 두 가지 결과 중 하나를 가질 수 있습니다.
///
/// Ok(T): 요소 T를 찾았습니다.
/// Err(E): 요소 E에서 오류가 발견되었습니다.
/// 관례적으로 예상되는 결과는 Ok이고 예상치 못한 결과는 Err입니다.
///
/// Option과 마찬가지로 Result에도 이와 관련된 많은 메서드가 있습니다. 예를 들어 unwrap()은 요소 T를 생성하거나 패닉을 발생시킵니다
/// 사례 처리의 경우 Result와 Option 사이에 겹치는 결합자가 많이 있습니다.
///
/// Rust로 작업하다 보면, Result 타입을 반환하는 메소드(parse() 메소드)를 접하게 될 것입니다
/// 문자열을 다른 유형으로 구문 분석하는 것이 항상 가능하지는 않을 수 있으므로 구문 분석()은 실패 가능성을 나타내는 결과를 반환합니다.
///
/// 문자열을 성공적으로 파싱하거나 실패했을 때 무슨 일이 일어나는지 봅시다:
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

/// 실패한 경우,parse()는 unwrap()이 패닉 상태가 되도록 하는 오류를 남깁니다
/// 게다가, 패닉은 프로그램을 종료하고 불쾌한 오류 메시지를 제공합니다.
///
/// 오류 메시지의 품질을 향상하려면 반환 유형을 보다 구체적으로 지정하고 오류를 명시적으로 처리하는 것을 고려해야 합니다.
fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}