// 실습.1
// enum IpAddrKind {
//     V4,
//     V6,
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// 실습.2
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// 실습.3
// 각 variant 는 다른 타입과 다른 양의 연관된 데이터를 갖는 실습
// V4 주소에 4개의 u8 값을 저장하길 원하지만, V6 주소는 하나의 String 값으로 표현 하자.
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// 실습.4
// variant 에 포함된 주소 데이터는 두 가지 다른 구조체로 되어, 각 variant 마다 다르게 정의 할 경우.
struct Ipv4Addr {
    // details elided
}

struct Ipv6Addr {
    // details elided
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// 실습.5
// 여러 타입의 variant 를 가질 경우
#[derive(Debug)]
enum Message {
    Quit, // Quit 은 연관된 데이터가 전혀 없습니다.
    Move {
        x: i32,
        y: i32,
    }, // Move 은 익명 구조체를 포함합니다.
    Write(String), // Write 은 하나의 String 을 포함합니다.
    ChangeColor(i32, i32, i32), // ChangeColor 는 세 개의 i32 을 포함합니다.
}

struct QuitMessage; // 유닛 구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체

impl Message {
    fn call(&self) {
        // 메소드 내용은 여기에 정의한다.
        println!("{:#?}", &self)
    }
}

// 실습.6
// Option 타입은 많이 사용되는데, 값이 있거나 없을 수도 있습니다.
// 이 개념을 타입 시스템의 관점으로 표현하자면, 컴파일러가 발생할 수 있는 모든 경우를 처리했는지 체크할 수 있습니다.
// 러스트는 다른 언어들에서 흔하게 볼 수 있는 null 특성이 없습니다.
// Null 은 값이 없다는 것을 표현하는 하나의 값입니다. null 을 허용하는 언어에서는,
// 변수는 항상 두 상태중 하나가 될 수 있습니다: null 혹은 null 이 아니다.

fn main() {
    // 실습.1
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // 실습.2
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    // 실습.3
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // 실습.5
    // let m = Message::Write(String::from("hello"));
    // m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    // Some 이 아닌 None 을 사용한다면, Option<T> 이 어떤 타입을 가질지 러스트에게 알려줄 필요가 있습니다.
    // 컴파일러는 None 만 보고는 Some variant 가 어떤 타입인지 추론할 수 없습니다.

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    //
    // let sum = x + y;
    // error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
    // not satisfied
    // y가 null 값이 들어올 수 있기 때문에, null 인경우의 로직이 명시적으로 존재해야 함. (match 로 해결)

}
