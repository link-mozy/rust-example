fn main() {
    // rust string 어려운 점:
    // 1. 가능한 에러를 꼭 노출하도록 하는 러스트의 성향
    // 2. 많은 프로그래머의 예상보다 더 복잡한 데이터 구조인 스트링
    // 3. UTF-8

    // 러스트는 핵심 언어 기능 내에서 딱 한가지 스트링 타입만 제공하는데, 이는 바로 스트링 슬라이스인 str 타입
    // 이것의 참조자 형태인 &str
    // 스트링 슬라이스에 대해 얘기했고, 이는 다른 어딘가에 저장된 UTF-8로 인코딩된 스트링 데이터의 참조자 이다.

    // String 타입은 핵심 언어 기능 내에 구현된 것이 아니고 러스트의 표준 라이브러리를 통해 제공되며,
    // 커질 수 있고, 가변적이며, 소유권을 갖고 있고, UTF-8로 인코딩된 스트링 타입입니다.
    // 러스트인들이 “스트링”에 대해 이야기할 때, 그들은 보통 String과 스트링 슬라이스 &str 타입 둘 모두를 이야기한 것이지,
    // 이들 중 하나를 뜻한 것은 아닙니다.

    // # 새로운 스트링 생성하기
    // let mut s = String::from();

    let data = "initial contents";
    let s1 = data.to_string();

    // the method also works on a literal directly:
    let s1 = "initial contents".to_string();

    // # 스트링 갱신하기
    let mut s2 = String::from("foo");
    s2.push_str("bar");

    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(&s4);
    println!("s4 is {}", s4);
    // string 붙인 후에도 s4를 사용할 수 잇다.

    let mut s5 = String::from("lo");
    s5.push_str("l");
    println!("s5 is {}", s5);
    // 한 글자도 추가가 가능하다.

    // # + 연산자나 format! 매크로를 이용한 접합
    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7; // s6는 여기서 이동되어 더이상 사용할 수 없다.
    println!("s8 is {}", s8);
    // println!("s6 is {}", s6); // value borrowed here after move
    // 여기서 + 는 add 메소드를 사용
    // fn add(self, s: &str) -> String {

    let s9 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");

    let s12 = s9 + "-" + &s10 + "-" + &s11;
    println!("{}", s12); // tic-tac-toe
    // s12의 결과 값을 생각하는데, 어렵다.
    // 우리는 format!를 사용하여 s12와 동일한 결과를 얻을 수 있다.
    let s9 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");

    let s12 = format!("{}-{}-{}", s9, s10, s11);
    println!("{}", s12); // tic-tac-toe

    // # 스트링 내부의 인덱싱
    // let s13 = String::from("hello");
    // let h = s13[0];
    // `String` cannot be indexed by `{integer}`
    // help: the trait `Index<{integer}>` is not implemented for `String`
    // 왜 String 타입은 인덱싱이 안되는 걸까?
    // 질문에 답변을 위해, 스트링이 메모리에 어떻게 저장되는지 알아야한다.

    // ## 내부적 표현
    // String은 Vec<u8>을 감싼 것입니다(wrapper).
    let len = String::from("Hola").len();
    // len 은 4이다. 각각의 글자가 1바이트 이기 때문이다. 그렇다면, 아래에서는 어떠할가?
    let len = String::from("Здравствуйте").len();
    println!("len is {}", len);
    // len 은 24가 나온다. 이는 위에 문자가 UTF-8 인코딩으로 저장되어, 저장소의 같이 2바이트로 되어있기 때문이다.

    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    // string indices are ranges of `usize`
    // 3의 byte 값은 208이지만, 기대치 않은 값을 반환하고 즉시 발견하기 힘들지도 모를 버그를 야기하는 것을 방지하기 위해,
    // 러스트는 이러한 코드를 전혀 컴파일하지 않고 이러한 오해들을 개발 과정 내에서 일찌감치 방지한다.

    // # 스트링 슬라이싱하기
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s is {}", s); // s is Зд
    // UTF-8은 2바이트씩 저장되므로 [0..2]일 경우 3이 출력된다.

    // # 스트링 내에서 반복적으로 실행되는 메소드
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
