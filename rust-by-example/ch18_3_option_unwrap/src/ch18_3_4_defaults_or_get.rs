/// Unpacking options and defaults
///
/// `Option`을 `unpack`하고 `None`인 경우 기본값으로 돌아가는 방법은 여러 가지가 있습니다.
/// 우리의 요구 사항을 충족하는 것을 선택하려면 다음을 고려해야 합니다.
///
/// 열정적인 평가가 필요한가요, 아니면 게으른 평가가 필요한가요?
/// 원래의 빈 값을 그대로 유지해야 합니까, 아니면 제자리에서 수정해야 합니까?

#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

#[cfg(test)]
mod tests {
    use super::*;

    /// `or()`는 연결 가능하고, 열심히 평가하며, 빈 값을 그대로 유지합니다.
    ///
    /// `or()`는 다음 예에서 볼 수 있듯이 연결 가능하며 인수를 열심히 평가합니다.
    /// `or`의 인수는 열심히 평가되므로 `or`에 전달된 변수가 이동됩니다.
    #[test]
    fn test_or() {
        let apple = Some(Fruit::Apple);
        let orange = Some(Fruit::Orange);
        let no_fruit: Option<Fruit> = None;

        let first_available_fruit = no_fruit.or(orange).or(apple);
        println!("first_available_fruit: {:?}", first_available_fruit);
        // first_available_fruit: Some(Orange)

        // `or`는 인수를 이동합니다.
        // 위의 예에서 `or(orange)`는 `Some`을 반환했기 때문에 `or(apple)`은 호출되지 않았습니다.
        // 하지만 `apple`이라는 변수는 관계없이 이동되어 더 이상 사용할 수 없습니다.
        // println!("Variable apple was moved, so this line won't compile: {:?}", apple);
        // TODO: 컴파일러 오류를 보려면 위 줄의 주석 처리를 제거하세요.
    }

    /// `or_else()`는 연결 가능하고 느리게 평가되며 빈 값을 그대로 유지합니다.
    /// 또 다른 대안은 'or_else'를 사용하는 것입니다. 이 역시 연결 가능하고 다음 예와 같이 느리게 평가됩니다
    #[test]
    fn test_or_else() {
        let no_fruit: Option<Fruit> = None;
        let get_kiwi_as_fallback = || {
            println!("Providing kiwi as fallback");
            Some(Fruit::Kiwi)
        };
        let get_lemon_as_fallback = || {
            println!("Providing lemon as fallback");
            Some(Fruit::Lemon)
        };

        let first_available_fruit = no_fruit
            .or_else(get_kiwi_as_fallback)
            .or_else(get_lemon_as_fallback);
        println!("first_available_fruit: {:?}", first_available_fruit);
        // 대체품으로 키위 제공
        // first_available_fruit: Some(Kiwi)
    }

    /// `get_or_insert()`는 열심히 평가하고 그 자리에서 빈 값을 수정합니다.
    ///
    /// `Option`에 값이 포함되어 있는지 확인하려면 다음 예와 같이 `get_or_insert`를 사용하여 대체 값으로 해당 값을 수정할 수 있습니다.
    /// `get_or_insert`는 매개변수를 열심히 평가하므로 `apple` 변수가 이동됩니다.
    #[test]
    fn test_get_or_insert() {
        let mut my_fruit: Option<Fruit> = None;
        let apple = Fruit::Apple;
        let first_available_fruit = my_fruit.get_or_insert(apple);
        println!("first_available_fruit is: {:?}", first_available_fruit);
        println!("my_fruit is: {:?}", my_fruit);
        // first_available_fruit is: Apple
        // my_fruit is: Some(Apple)
        //println!("Variable named `apple` is moved: {:?}", apple);
        // TODO: 컴파일러 오류를 보려면 위 줄의 주석 처리를 제거하세요.
    }

    /// `get_or_insert_with()`는 느리게 평가되고, 빈 값을 그 자리에서 수정합니다.
    ///
    /// 대체할 값을 명시적으로 제공하는 대신 다음과 같이 `get_or_insert_with`에 클로저를 전달할 수 있습니다.
    #[test]
    fn test_get_or_insert_with() {
        let mut my_fruit: Option<Fruit> = None;
        let get_lemon_as_fallback = || {
            println!("Providing lemon as fallback");
            Fruit::Lemon
        };
        let first_available_fruit = my_fruit
            .get_or_insert_with(get_lemon_as_fallback);
        println!("first_available_fruit is: {:?}", first_available_fruit);
        println!("my_fruit is: {:?}", my_fruit);
        // 레몬을 대체물로 제공
        // first_available_fruit is: Lemon
        // my_fruit is: Some(Lemon)

        // Option에 값이 있으면 변경되지 않고 그대로 유지되며 클로저가 호출되지 않습니다.
        let mut my_apple = Some(Fruit::Apple);
        let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
        println!("should_be_apple is: {:?}", should_be_apple);
        println!("my_apple is unchanged: {:?}", my_apple);
        // 출력은 다음과 같습니다. 'get_lemon_as_fallback' 클로저는 호출되지 않습니다.
        // should_be_apple is: Apple
        // my_apple is unchanged: Some(Apple)
    }
}