struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[allow(unused)]
#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {

    // 해당 사람의 직장 전화번호(존재하는 경우)의 지역 번호를 가져옵니다.
    fn work_phone_area_code(&self) -> Option<u8> {
        // `?` 연산자 없이 중첩된 `match` 문이 많이 필요합니다.
        // 훨씬 더 많은 코드가 필요합니다. 직접 작성해 보고 어떤 것이 있는지 확인하세요.
        // 더 쉽습니다.
        self.job?.phone_number?.area_code
    }
}

fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

#[test]
fn test_next_birthday() {
    assert_eq!(next_birthday(Some(30)), Some("Next year I will be 31".to_string()));
    assert_eq!(next_birthday(None), None);
}

#[test]
fn test_work_phone_area_code() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };
    assert_eq!(p.work_phone_area_code(), Some(61));

    let p = Person {
        job: Some(Job {
            phone_number: None,
        }),
    };
    assert_eq!(p.work_phone_area_code(), None);

    let p = Person {
        job: None,
    };
    assert_eq!(p.work_phone_area_code(), None);
}