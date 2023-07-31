fn main() {
    // 튜플 만들기
    let banana = ("바나나", 300);
    let apple = ("사과", 200);

    // 튜플을 참조해 합계 금액 구하기
    let total = banana.1 + apple.1;

    // 튜플 내용 표시
    print_tuple(&banana);
    print_tuple(&apple);
    println!("합계는 {}원입니다.", total);
}

fn print_tuple(item: &(&str, i64)) {
    println!("{}를 {}원에 구입.", item.0, item.1);
}