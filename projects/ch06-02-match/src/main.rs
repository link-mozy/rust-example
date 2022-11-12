// 실습.1
// enum Coin {
//     Penny,
//     Nickle,
//     Dime,
//     Quarter,
// }
//
// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickle => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// if 와 차이점은 enum 은 어떤 타입에서든 사용 가능.
// if를 사용하는 경우, 해당 표현식은 부울린 값을 반환할 필요가 있습니다. 여기서는 어떤 타입이든 가능합니다.

// match 에서 로직을 실행할 경우 중 괄호 {} 를 사용.
// ex.
// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickle => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// 실습.2
// ## 값들을 바인딩하는 패턴들
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}", state);
            25
        },
    }
}

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is {:?}", six);
    println!("none is {:?}", none);

    // Placeholder (변경자) : _
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    // some_u8_value 는 1,3,5,7 에 해당되지 않아 _ => () 이다. (0)
}
