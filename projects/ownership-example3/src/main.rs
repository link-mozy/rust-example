fn main() {
    // # 슬라이스(Slices)
    // let mut s = String::from("hello world");
    //
    // let word = first_word(&s); // word는 5를 갖게 될 것입니다.
    //
    // println!("{}", word);
    //
    // s.clear(); // 이 코드는 String을 비워서 ""로 만들게 됩니다.
    //
    // // word는 여기서 여전히 5를 갖고 있지만, 5라는 값을 의미있게 쓸 수 있는 스트링은 이제 없습니다.
    // // word는 이제 완전 유효하지 않습니다!

    // # 스트링 슬라이스
    // let s = String::from("hello world");
    //
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // println!("hello: {}", hello);
    // println!("world: {}", world);

    // let s = String::from("hello");
    //
    // let slice = &s[0..2];
    // println!("slice : {}", slice);
    // let slice = &s[..2];
    // println!("slice : {}", slice);

    // let s = String::from("hello");
    // let len = s.len();
    // let slice = &s[3..len];
    // println!("slice : {}", slice);
    // let slice = &s[3..];
    // println!("slice : {}", slice);

    // let s = String::from("hello");
    // let len = s.len();
    // let slice = &s[0..len];
    // println!("slice : {}", slice);
    // let slice = &s[..];
    // println!("slice : {}", slice);

    // let mut s = String::from("hello world");
    //
    // let word = first_word(&s);
    //
    // s.clear();
    //
    // println!("the first word is: {}", word);
    // // let mut s 는 가변 변수라 에러가 발생
    // // ---- immutable borrow later used here

    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작합니다.
    let word = first_word(&my_string);
    println!("word : {}", word);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);
    println!("word : {}", word);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word = first_word(my_string_literal);
    println!("word : {}", word);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for(i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

// fn first_word(s: &String) -> &str { // &str : "스트링 슬라이스"를 나타내는 타입
fn first_word(s: &str) -> &str { // &String == &str : &String과 &str 둘 모두에 대한 같은 함수를 사용할 수 있다.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
