#![allow(dead_code)]

#[derive(Debug)] enum Food { Apple, Carrot, Potato }

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

// 음식 껍질 벗기기. 아무것도 없으면 `None`을 반환합니다.
// 그렇지 않으면 껍질을 벗긴 음식을 반환합니다.
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None       => None,
    }
}

// 음식을 자르는 중입니다. 아무것도 없으면 `None`을 반환합니다.
// 그렇지 않으면 다진 음식을 반환합니다.
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None               => None,
    }
}

/// `match`는 `Options`를 처리하는 유효한 방법입니다.
/// 그러나 결국 과도한 사용은 지루함을 느낄 수 있습니다. 특히 입력에만 유효한 작업의 경우 더욱 그렇습니다.
/// 이러한 경우 '결합자'를 사용하여 모듈 방식으로 제어 흐름을 관리할 수 있습니다.
///
/// `Option`에는 `Some -> Some` 및 `None -> None`의 간단한 매핑을 위한 결합자인 `map()`이라는 내장 메서드가 있습니다. 유연성을 높이기 위해 여러 `map()` 호출을 함께 연결할 수 있습니다.
///
/// 다음 예에서 `process()`는 간결함을 유지하면서 이전의 모든 함수를 대체합니다:
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// 음식의 껍질을 벗기고, 자르고, 요리하는 과정을 순서대로 수행하는 기능입니다.
// 코드를 단순화하기 위해 `map()`을 여러 번 사용하도록 연결합니다.
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// 먹어보기 전에 음식이 있는지 확인해보세요!
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None       => println!("Oh no! It wasn't edible."),
    }
}

fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // 이제 더 간단해 보이는 `process()`를 시도해 보겠습니다.
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}