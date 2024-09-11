// Using Result in main
/// 명시적으로 지정된 경우 결과 유형은 기본 함수의 반환 유형이 될 수도 있습니다
/// 일반적으로 주요 기능은 다음과 같은 형식입니다.

/// 그러나 main은 Result의 반환 유형도 가질 수 있습니다
/// 메인 함수 내에서 오류가 발생하면 오류 코드를 반환하고 (Debug 특성을 사용하여) 오류의 디버그 표현을 인쇄합니다
/// 다음 예에서는 이러한 시나리오를 보여주고 다음 섹션에서 다루는 측면을 다룹니다.
use std::num::ParseIntError;



fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}