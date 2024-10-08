mod _1_unpacking_options_with;
mod _2_combinators_map;
mod _3_combinators_and_then;
mod _4_defaults_or_get;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod option_unwrap_test {

    fn give_adult(drink: Option<&str>) {
        match drink {
            Some("lemonade") => println!("Yuck! Too sugary."),
            Some(inner)   => println!("{}? How nice.", inner),
            None          => println!("No drink? Oh well."),
        }
    }

    #[test]
    fn test_give_adult() {
        let water  = Some("water");
        let lemonade = Some("lemonade");
        let void  = None;

        // water? How nice.
        give_adult(water);
        // Yuck! Too sugary.
        give_adult(lemonade);
        // No drink? Oh well.
        give_adult(void);
    }

    fn drink(drink: Option<&str>) {
        let inside = drink.unwrap();
        if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

        println!("I love {}s!!!!!", inside);
    }

    #[test]
    #[should_panic]
    fn test_drink() {
        let coffee = Some("coffee");
        let nothing = None;

        // I love coffees!!!!!
        drink(coffee);
        // thread 'option_unwrap_test::test_drink' panicked at src/main.rs:36:28:
        // called `Option::unwrap()` on a `None` value
        drink(nothing);
    }
}
