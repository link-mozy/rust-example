// 1에서 75까지의 숫자가 들어간 리스트를 만든다.
// 리스트의 항목을 섞는다.
// 24개의 요소를 꺼내 빙고 카드에 할당한 뒤 출력한다.

// 배열을 섞기 위한 rand 크레이트 이용 선어
use rand::seq::SliceRandom;

fn main() {
    // 1에서 75까지의 숫자로 이루어진 배열을 생성
    let mut nums = [0; 75];
    for i in 1..=75 {nums[i-1] = i;};

    // 섞기
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    // 카드 표시 [ 5x5 배열 ]
    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 { // 와일드 카드
                print!("  *,");
            } else {
                print!("{:3},", nums[i]);
            }
        }
        println!("");
    }
}