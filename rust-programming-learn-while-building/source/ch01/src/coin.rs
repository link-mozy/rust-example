// 문제
// 어느 가게의 계산 카운터에 500원짜리 10개, 100원짜리 3개, 50원짜리가 10개가 있다.
// 잔돈으로 3950원을 거슬러 줘야 할 경우 나올 수 있는 모든 조합을 나열하시오.

// 러스트로 거스름돈 조합 계산
fn main() {
    println!("프로그램 시작!");
    // 거스름돈
    let price = 3950;
    // 500원 동전의 수만큼 반복
    for i500 in 0..11 {
        // 100원 동전의 수만큼 반복
        for i100 in 0..4 {
            // 50원 동전의 수만큼 반복
            for i50 in 0..11 {
                // 동전의 합계를 계산
                let total = i50 * 50 + i100 * 100 + i500 * 500;
                // 동전의 합계가 거스름돈과 같으면 출력
                if price == total {
                    println!("500원x{}+100원x{}+50원x{}={}",
                    i500, i100, i50, total);
                }
            }
        }
    }
}