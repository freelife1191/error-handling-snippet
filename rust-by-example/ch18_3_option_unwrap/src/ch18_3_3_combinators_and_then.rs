/// `map()`은 match 문을 단순화하기 위한 연결 가능한 방법으로 설명되었습니다.
/// 그러나 `Option<T>`를 반환하는 함수에 `map()`을 사용하면 중첩된 `Option<Option<T>>`이 발생합니다.
/// 여러 호출을 함께 연결하면 혼란스러울 수 있습니다. 여기에는 일부 언어에서는 플랫맵으로 알려진 `and_then()`이라는 또 다른 결합자가 사용됩니다.
///
/// `and_then()`은 래핑된 값으로 함수 입력을 호출하고 결과를 반환합니다.
/// `Option`이 `None`이면 대신 `None`을 반환합니다.
///
/// 다음 예에서 `cookable_v3()`의 결과는 `Option<Food>`입니다.
/// `and_then()` 대신 `map()`을 사용하면 `eat()`에 유효하지 않은 유형인 `Option<Option<Food>>`이 제공됩니다.

#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// 스시를 만들 재료가 없습니다.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// 꼬르동 블루를 제외한 모든 요리의 레시피가 있습니다.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

// 요리를 만들려면 레시피와 재료가 모두 필요합니다.
// `일치' 체인으로 논리를 표현할 수 있습니다.
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None       => None,
        Some(food) => have_ingredients(food),
    }
}

// `and_then()`을 사용하면 더 간결하게 편리하게 다시 작성할 수 있습니다.
fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

// 그렇지 않으면 `Option<Option<Food>>`를 `Flatten()`해야 합니다.
// `Option<Food>`를 얻으려면:
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}