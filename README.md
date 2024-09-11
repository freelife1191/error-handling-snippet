# Error handling

https://doc.rust-lang.org/stable/rust-by-example/error.html#error-handling

오류 처리는 실패 가능성을 처리하는 프로세스입니다. 예를 들어, 파일을 읽지 못하고 잘못된 입력을 계속 사용하는 것은 확실히 문제가 될 수 있습니다.
이러한 오류를 인지하고 명시적으로 관리하면 프로그램의 나머지 부분이 다양한 함정으로부터 보호됩니다.

Rust에는 오류를 처리하는 다양한 방법이 있으며, 이에 대해서는 다음 하위 장에서 설명합니다.
그것들은 모두 다소 미묘한 차이와 사용 사례가 다릅니다. 경험상 다음과 같습니다.

명시적인 '패닉'은 주로 테스트 및 복구할 수 없는 오류 처리에 유용합니다.
예를 들어 아직 구현되지 않은 함수를 처리할 때 프로토타입을 만드는 데 유용할 수 있지만 이러한 경우 더 설명적인 '구현되지 않음'이 더 좋습니다.
테스트에서 `패닉`은 명시적으로 실패하는 합리적인 방법입니다.

`Option` 유형은 값이 선택적이거나 값이 부족하여 오류 조건이 아닌 경우에 사용됩니다.
예를 들어 디렉토리의 상위 디렉토리인 `/`와 `C:`에는 디렉토리가 없습니다.
`옵션`을 다룰 때 `unwrap`은 프로토타입 제작 및 값이 보장되는 경우에 적합합니다.
그러나 `expect`는 어쨌든 문제가 발생할 경우 오류 메시지를 지정할 수 있으므로 더 유용합니다.

일이 잘못되어 호출자가 문제를 처리해야 할 가능성이 있는 경우 `Result`를 사용하세요.
당신은 그것들을 `풀고` `기대`할 수도 있습니다(테스트나 빠른 프로토타입이 아닌 이상 그렇게 하지 마십시오).

오류 처리에 대한 보다 엄격한 논의를 보려면 '공식 도서'의 오류 처리 섹션을 참조하세요.