/// 다음은 drink() 다시 작성에 초점을 맞추고 unwind 키워드를 명시적으로 사용하는 또 다른 예입니다.
#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!!!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!!!!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

/// 패닉 전략은 `abort` 또는 `unwind`를 사용하여 명령줄에서 설정할 수 있습니다.
/// rustc  lemonade.rs -C panic=abort
#[cfg(test)]
mod unwind_test {
    #[test]
    fn test_drink() {
        super::drink("water");
        super::drink("lemonade");
    }
}