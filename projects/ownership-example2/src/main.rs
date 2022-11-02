fn main() {
    // println!("참조자(References)와 빌림(Borrowing)");

    // References example
    // let s1 = String::from("hello");
    //
    // let len = calculate_length(&s1);
    //
    // println!("The length of '{}' is {}.", s1, len);

    // Borrowing example
    // let s = String::from("hello");
    //
    // change(&s);
    // 변수가 기본적으로 불변인 것처러면, 참조자도 마찬가지라 에러가 난다.

    // ## 가변 참조자(Mutable References)
    // let mut s = String::from("hello");
    //
    // change(&mut s);

    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s;
    // 한번 참조한뒤 다시 참조하는 것을 러스트가 허용하지 않는다.
    // 이러한 제한이 가지는 이점은 바로 러스트가 컴파일 타임에 데이터 레이스(data race)를 방지할 수 있도록 해준다는 것입니다.

    // 데이터 레이스 조건:
    // 1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
    // 2. 그 중 적어도 하나의 포인터가 데이터를 쓴다.
    // 3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.

    // 참조를 여러번 하는 방법
    // let mut s = String::from("hello");
    //
    // {
    //     let r1 = &mut s;
    // } // 여기서 r1은 스코프 밖으로 벗어났으므로, 우리는 아무 문제 없이 새로운 참조자를 만들 수 있다.
    //
    // let r2 = &mut s;

    // let mut s = String::from("hello");
    //
    // let r1 = &s; // 문제 없음
    // let r2 = &s; // 문제 없음
    // let r3 = &mut s; // 큰 문제
    // 우리는 불변 참조자를 가지고 있을 동안에도 역시 가변 참조자를 만들 수 없는데,
    // 불변 참조자의 사용자는 사용중인 동안에 값이 갑자기 바뀌리라 예상하지 않기 때문이다.
    // 하지만 여러 개의 불변 참조자는 만들 수 있는데,
    // 데이터를 그냥 읽기만하는 것은 다른 것들이 그 데이터를 읽는데에 어떠한 영향도 주지 못하기 때문이다.

    // ## 댕글리 참조자(Dangling References)
    // 댕글리 참조자? 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게 사용하도록
    // 줘버렸을 지도 모를 메모리를 참조하는 포인터를 말한다.
    // 러스트에서는 컴파일러가 모든 참조자들이 댕글링 참조자가 되지 않도록 보장해 준다.
    // let reference_to_nothing = dangle();
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // (해석: 이 함수의 반환 타입은 빌린 값을 포함하고 있는데, 빌려온 실제 값은 없습니다.)

    let reference_to_nothing = no_dangle();

    // ## 참조자의 규칙
    // 우리가 참조자에 대해 논의한 것들을 정리
    // 1. 어떠한 경우이든 간에, 여러분은 아래 둘 다는 아니고 둘 중 하나만 가질 수 있습니다:
    //   * 하나의 가변 참조자
    //   * 임의 개수의 불변 참조자
    // 2. 참조자는 항상 유효해야만 한다.
}

fn calculate_length(s: &String) -> usize { // s는 String의 참조자입니다
    // 이 엠퍼센드(&) 기호가 참조자이며, 이는 여러분이 어떤 값을 소유권을 넘기지 않고 참조할수 있도록 해준다.
    // 다이어 그램:
    // s : ptr --> s1 : ptr --> hello:String [idx]
    s.len()
} // 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기
  // 때문에, 아무런 일도 발생하지 않습니다.

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // 우리가 참조하는 어떤 것을 변경하는 것은 허용되지 않는다.
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle은 String의 참조자를 반환합니다
//     let s = String::from("hello"); // s는 새로운 String입니다
//     &s // 우리는 String s의 참조자를 반환합니다.
// } // 여기서 s는 스코프를 벗어나고 버려집니다. 이것의 메모리는 사라집니다.
// 위험하군요!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}