// 피보나치 수열 fib
fn fib(n:i64) -> i64 {
    // 재귀 탈출 조건
    if n == 1 { return 0; }
    if n == 2 { return 1; }

    fib(n - 2) + fib(n - 1)
}

fn main() {
    for i in 2..=20 {
        println!("{}: {}", i, fib(i));
    }
}