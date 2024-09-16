#[allow(unused)]
#[derive(Debug, Copy, Clone)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

#[test]
#[allow(unused)]
fn test_or() {
    let apple: Option<Fruit> = Some(Fruit::Apple);
    let orange: Option<Fruit> = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    // or는 가장 처음 true 인 값 반환 and 는 모두 true 일때 반환
    let first_available_fruit = no_fruit.or(orange).or(apple);
    // and 는 모두 true 일때 가장 마지막 값 반환 None 이 있으면 None 반환
    let all_available_fruit = apple.and(no_fruit).and(orange);
    println!("first_available_fruit: {:?}", first_available_fruit);
    println!("all_available_fruit: {:?}", all_available_fruit);
}


// or_else 는 or 와 비슷하지만 or_else 는 클로저를 받아서 클로저의 반환값을 반환한다.
#[test]
#[allow(unused)]
fn test_or_else() {
    let no_fruit: Option<Fruit> = None;
    let orange = || Some(Fruit::Orange);
    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(orange)
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit);
}


// get_or_insert 는 Option 이 None 일때 대체값을 넣어주는 함수이다.
#[test]
#[allow(unused)]
fn test_get_or_insert() {
    let mut fruit = Some(Fruit::Apple);
    let mut no_fruit: Option<Fruit> = None;
    // fruit 가 Apple 이므로 None 이 아니라서 Applie 를 그대로 반환
    let apple = fruit.get_or_insert(Fruit::Lemon);
    println!("apple: {:?}", apple);
    // no_fruit 가 None 이므로 Orange 를 대체값으로 넣어준다.
    let orange = no_fruit.get_or_insert(Fruit::Orange);
    println!("orange: {:?}", orange);
}

// get_or_insert_with 는 Option 이 None 일때 대체값을 넣어주는 함수이다.
// get_or_insert 와 다른점은 대체값을 클로저로 받는다.
#[test]
#[allow(unused)]
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