fn main() {
    let s = "안녕하세요";
    // 첫 1글자를 가져온다.
    let ch = s.chars().nth(0).unwrap();
    println!("{}", ch); // 안
    // 3번째 글자를 가져온다.
    let ch = s.chars().nth(2).unwrap();
    println!("{}", ch); // 하
}