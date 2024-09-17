use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError {
    vec: Vec<String>
}

// struct 는 인터페이스 역할을 하며 fmt::Display 트레이트를 상속받아서 기본 메소드를 오버라이딩 한다
// 이때 self 값으로 원하는 값을 전달할 수 있다
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}", "invalid first item to double", &self.vec.join(", "))
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // 오류를 새로운 유형으로 변경합니다.
        .ok_or(DoubleError { vec: vec.iter().map(|s| s.to_string()).collect() })
        .and_then(|s| {
            s.parse::<i32>()
                // 여기에서도 새로운 오류 유형으로 업데이트됩니다.
                .map_err(|_| DoubleError { vec: vec.iter().map(|s| s.to_string()).collect() } )
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn test_double_first() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    // The first doubled is 84
    print(double_first(numbers));
    // Error: invalid first item to double ""
    print(double_first(empty));
    // Error: invalid first item to double "tofu, 93, 18"
    print(double_first(strings));
}