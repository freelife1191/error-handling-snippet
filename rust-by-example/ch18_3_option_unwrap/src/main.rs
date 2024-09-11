/// 마지막 예에서는 프로그램 실패를 마음대로 유도할 수 있음을 보여주었습니다.
/// 우리는 달콤한 레모네이드를 마시면 '패닉'하도록 프로그램에 지시했습니다.
/// 하지만 음료수를 기대했는데 받지 못했다면 어떻게 될까요? 그 경우도 마찬가지이므로 처리해야 합니다!
///
/// 레모네이드와 마찬가지로 null 문자열("")에 대해 테스트할 수 있습니다.
/// 우리는 Rust를 사용하고 있으므로 대신 컴파일러가 음료수가 없는 경우를 지적하도록 합시다.
///
/// std 라이브러리의 `Option<T>`라는 `enum`은 부재가 가능할 때 사용됩니다.
/// 두 가지 '옵션' 중 하나로 나타납니다:
///
/// Some(T): An element of type T was found
/// None: No element was found
///
/// 이러한 경우는 일치를 통해 명시적으로 처리하거나 `unwrap`을 사용하여 암시적으로 처리할 수 있습니다.
/// 암시적 처리는 내부 요소 또는 `패닉`을 반환합니다.
///
/// `expect`를 사용하여 `panic`을 수동으로 사용자 정의하는 것이 가능하지만 `unwrap`은 명시적 처리보다 덜 의미 있는 출력을 남깁니다.
/// 다음 예에서 명시적 처리는 원하는 경우 `패닉` 옵션을 유지하면서 보다 제어된 결과를 산출합니다.

mod ch18_3_1_unpacking_options_with;
mod ch18_3_3_combinators_and_then;
mod ch18_3_4_defaults_or_get;
mod ch18_3_2_combinators_map;

// 어른은 모든 것을 보았고 어떤 '음료'도 잘 다룰 수 있습니다.
// 모든 `drinks`는 `match`를 사용하여 명시적으로 처리됩니다.
fn give_adult(drink: Option<&str>) {
    // 각 사례에 대한 조치 과정을 지정합니다.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No drink? Oh well."),
    }
}

// 다른 사람들은 달콤한 `drinks`를 마시기 전에 `panic`할 것입니다.
// 모든 `drinks`는 `unwrap`을 사용하여 암시적으로 처리됩니다.
fn drink(drink: Option<&str>) {
    // `unwrap`은 `None`을 수신하면 `panic`을 반환합니다.
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}