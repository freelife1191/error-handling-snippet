/// 우리가 보게 될 가장 간단한 오류 처리 메커니즘은 패닉입니다.
/// 오류 메시지를 인쇄하고 스택 풀기를 시작하며 일반적으로 프로그램을 종료합니다.
/// 여기서는 오류 조건에 대해 명시적으로 패닉을 호출합니다.
fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

/// drink에 대한 첫 번째 호출이 작동합니다. 두 번째 패닉이 발생하므로 세 번째는 호출되지 않습니다.
fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}