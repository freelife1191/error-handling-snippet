#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None       => None,
        Some(food) => have_ingredients(food),
    }
}

// and_then 은 함수를 호출해 None 이면 None 반환 아니면 함수의 결과를 반환
fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

// map 은 Option 을 한번 더 감싸므로 한꺼풀을 벗겨내려면 flatten 을 사용해야 한다
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn eat_v2(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn eat_v1(food: Food, day: Day) {
    match cookable_v1(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

#[test]
#[allow(unused)]
fn test_cookable_v1() {

    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
    // Oh no. We don't get to eat on Monday?
    eat(cordon_bleu, Day::Monday);
    // Yay! On Tuesday we get to eat Steak.
    eat(steak, Day::Tuesday);
    // Oh no. We don't get to eat on Wednesday?
    eat(sushi, Day::Wednesday);

    // eat_v2(cordon_bleu, Day::Monday);
    // eat_v2(steak, Day::Tuesday);
    // eat_v2(sushi, Day::Wednesday);

    // eat_v1(cordon_bleu, Day::Monday);
    // eat_v1(steak, Day::Tuesday);
    // eat_v1(sushi, Day::Wednesday);
}