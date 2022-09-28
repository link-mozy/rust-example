use std::io;

fn main() {
    // 피보나치수열 : 앞 두 개의 항목의 합이 그다음 항목의 값이 되는 수열을 의미한다.
    // 실습:
    // 0부터 시작하여 사용자가 입력한 정수값보다 작은 피보나치 수를 출력하는 프로그램을 만들어 보자.
    let mut pre = 0;
    let mut next = {
        pre + 1
    };
    let mut input_number = String::new();

    println!("Please input number : ");
    io::stdin().read_line(&mut input_number).expect("Failed to read line");
    let mut input_number: i32 = input_number.trim().parse().expect("Please type a number!");

    loop {
        let result = {
            pre + next
        };
        if input_number < result {
            break;
        }
        pre = next;
        next = result;
        println!("{}!", result);
    }
}
