fn main() {
    // 러스트는 제 3의 접근법을 이용
    // 메모리는 컴파일 타임에 컴파일러가 체크할 규칙들로 구성된 소유권 시스템을 통해 관리
    // 소유권 기능들의 어던 것도 런타임 비용이 발생하지 않는다.
    // 스택과 힙 둘다 런타임에 사용할 수 잇는 메모리 부분
    // 스택의 특징 LIFO 때문에 데이터 접근이 빠름 (데이터 크기가 고정)
    // 컴파일 타임에 크기가 결정되어 있지 않거나 크기가 변경될 수 잇는 데이터를 위해서는, 힙에 데이터를 저장할 수 잇다.
    // 데이터를 힙에 넣을때, 먼저 저장할 공간이 있는지 물어본다.
    // 그러면 운영체제가 충분히 커다란 힙 안의 빈 어떤 지점을 찾아서 이 곳을 사용중이라고 표시하고,
    // 해당 지점의 포인터를 우리에게 돌려주준다. 이절차를 `힙 공간 할당하기(allocating on the heap)`라고 부르고,
    // 종종 그냥 "할당(allocating)"으로 부른다.
    // 스택에 포인터를 푸싱하는 것은 할당에 해당되지 않는다. 포인터는 결정되어 있는 고정된 크기의 값이므로,
    // 우리는 스택에 포인터를 저장할 수 잇지만, 실제 데이터를 사용하고자 할 때는 포인터를 따라가야 한다.

    // ## 소유권 규친
    // 소유권의 규칙:
    // 1. 러스트의 각각의 값은 해당값의 오너(`owner`)라고 풀리우는 변수를 갖고 있다.
    // 2. 한번에 딱 하나의 오너만 존자할 수 있다.
    // 3. 오너가 스코프 밖으로 벗어나는 대, 값은 버려진다(dropped).

    // ## String type
    // let mut s = String::from("hello");
    //
    // s.push_str(", world!"); // push_str() 은 해당 스트링 리터럴을 스트링에 붙여준다.
    //
    // println!("{}", s);

    // ## 메모리 할당
    // 러스트는 스코프 밖으로 벗어나는 순간 자동으로 메모리 할당 해제 (rust는 `drop`이라는 함수를 호출하여 안쓰는 메모리 공간 반환)

    // ## 변수와 데이터가 상호작용하는 방법: 이동(move)
    // 얕은 복사 후 메모리를 해제하려고할때, 러스트는 자동적으로 `drop` 함수를 호출하여 해당 변수가 사용하는 힙 메모리를 제거
    // 하지만, 두 데이터가 모두 같은 곳을 가리키고 있다. 둘 다 같은 메로리를 해제하려 하기 때문
    // 이는 두번 해제(double free) 오류라고 알려져 있으며 이전에 언급한 바 있는 메모리 안정성 버그들 중 하나
    // 메모리를 두번 해제하는 것은 메모리 송산(memory corruption)의 원인이 되는데, 이는 보안 취약성 문제를 일으킬 가능성이 있다.
    // ex.
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world", s1);
    // output:
    // error[E0382]: borrow of moved value: `s1`
    // 37 |     let s1 = String::from("hello");
    // |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    // 38 |     let s2 = s1;
    // |              -- value moved here
    // 39 |     println!("{}, world", s1);
    // |                           ^^ value borrowed here after move
    // |
    // = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
    //
    // 다른 언어로 프로그래밍 하는 동안 "얕은 복사(shallow copy)"와 "깊은 복사(deep copy)"라는 용어를 배웠을 거다.
    // 데이터의 복사 없이 포인터와 길이값 및 용량값만 복사하는 개념이 얕은 복사와 비슷하지만, 러스트는 첫번째 변수를 무효화 시키기도 한다.
    // 얕은 복사라고 부르는 대신 `이동(move)`이라고 말한다.

    // ## 변수와 데이터가 상호작용하는 방법: 클론
    // 만일 string 의 스택 데이터 만이 아니라, 힙 데이터를 깊이 복사하기를 정말 원한다면, `clone`이라 불리우는 공용 메소드를 사용할 수 있다.
    // ex.
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // ## 스택에만 있는 데이터: 복사
    // `Copy` 가 가능한 몇가지 타입:
    // * `u32`와 같은 모든 정수형 타입들
    // * `true`와 `false`값을 갖는 부울린 타입 `bool`
    // * `f64`와 같은 모든 부동 소수점 타입들
    // * `Copy`가 가능한 타입만으로 구성된 튜플들. `(i32, i32)`는 `Copy`가 되지만, `(i32, String)`은 안된다.

    // ## 소유권과 함수
    // let s = String::from("hello");    // s가 스코프 안으로 들어왔다.
    //
    // takes_ownership(s);                         // s의 같이 함수 안으로 이동했다...
    //                                             // ... 그리고 이제 더이상 유효하지 않는다.
    // let x = 5;                             // x가 스코프 안으로 들어왔다.
    //
    // make_copy(x);                              // x가 함수 안으로 이동했다마, i32는 Copy가 되므로, x를 이후에도 사용 가능
    // // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나간다. 하지만 s는 이미 이동되었으므로, 별다른 일이 발생되지 않는다.

    // ## 반환 값과 스코프
    // let s1 = gives_ownership();                 // gives_ownership은 반환값을 s1에게 이동
    //
    // let s2 = String::from("hello");   // s2가 스코프 안으로 들어왔다.
    //
    // let s3 = takes_and_gives_back(s2);          // s2는 takes_and_gives_back 안으로 이동, 이 함수가 반환값을 s3으로 이동
    // println!("{}", s3);

    // 힙에 데이터를 갖고 있는 변수가 스코프 밖으로 벗어나면, 해당 값은 데이터가 다른 변에 의해 소유되도 이동하지 않는한 `drop`에 의해 제거될 것이다.

    // 만일, 함수에게
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔다.
    println!("{}", some_string);
    // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출된다. 메모리는 해제되었다.
}

fn make_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔다.
    println!("{}", some_integer);
    // 여기서 some_integer가 스코프 밖으로 벗어났다. 별다른 일은 발생하지 않는다.
}

fn gives_ownership() -> String { // gives_ownership 함수가 반환 값을 호출한 쪽으로 이동
    let some_string = String::from("hello");

    some_string
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환한다. // 내용은 같은 새로 값을 주는 개념같다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로 들어왔다.

    a_string // a_string은 반환되고, 호출한 쪽의 함수로 이동
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}