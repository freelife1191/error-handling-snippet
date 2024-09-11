/// 이전 섹션에서는 오류 처리 메커니즘 패닉을 보여줍니다.
/// 패닉 설정에 따라 다양한 코드 경로를 조건부로 컴파일할 수 있습니다. 현재 사용 가능한 값은 'unwind' 및 'abort'입니다.
///
/// 이전 레모네이드 예시를 바탕으로 우리는 명시적으로 패닉 전략을 사용하여 다양한 코드 줄을 연습합니다.
pub fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!!");
        } else {
            println!("Spit it out!!!!");
        }
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

#[cfg(test)]
mod abort_test {
    #[test]
    fn test_drink() {
        super::drink("water");
        super::drink("lemonade");
    }
}