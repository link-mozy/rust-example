pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;
// or use a::series::of::nested_modules;
// 로 사용하면, nested_modules(); 만 사용 가능!

enum TrafficLight {
    Red,
    Yellow,
    Green,
}
// 열거형도 쌉가능!
use TrafficLight::{Red, Yellow};

// *를 이용한 모든(glob) 가죠오기
enum Animal {
    Dog,
    Cat,
}
use Animal::*;

fn main() {
    // a::series::of::nested_modules();
    // 너무 길어 rust에서는 간결하게 불러오도록 도와주는 use를 사용.
    of::nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    let dog = Dog;
    let cat = Cat;
}
