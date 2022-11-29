// panic! 메크로
// panic 메크로가 실행되면, 여러분의 프로그램은 실패 메세지를 출력하고, 스택을 되감고 청소하고, 그 후 종료됩니다.
// 이런 일이 발생하는 가장 흔한 상황은 어떤 종류의 버그가 발견되었고 프로그래머가 이 에러를 어떻게 처리할지가 명확하지 않을 때 입니다.

// panic 의 종류 :
// * 되감기 (unwinding)
// * 그만두기 (abort)
// 만약, 그만두기로 설정하고 싶다면, Cargo.toml 내에서 [profile] 섹션에 `panic = 'abort'`를 추가

fn main() {
    // 단순한 프로그램 내에서 `panic!` 호출 하기
    // panic!("crash and burn");

    // panic! 백트레이스 사용하기
    let v = vec![1, 2, 3];
    v[99];
}
