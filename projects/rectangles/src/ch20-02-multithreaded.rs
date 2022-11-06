// 구조체를 이용한 예제 프로그램
// 사각형의 넓이를 계산하는 프로그램

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
// 러스트는 우리를 위해 derive 어노테이션을 이용한 여러 트레잇을 제공하여 우리의 커스텀 타입에 대해 유용한 동작을 추가할 수 있도록 해줍니다.


fn main() {
    // let length1 = 50;
    // let width1 = 30;
    //
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(length1, width1)
    // );

    // let rect1 = (50, 30);
    //
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // let rect1 = Rectangle { length: 50, width: 30 };
    //
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:#?}", rect1);
}

// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle : Rectangle) -> u32 {
    rectangle.length * rectangle.width
}