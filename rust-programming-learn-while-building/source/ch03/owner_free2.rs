fn main() {
    // 블록1
    {
        let s1 = String::from("인생에 뜻을 세우는 데 있어 늦은 떼라곤 없다.");
        let s3 = String::from("계단을 밟아야 계단 위에 올라설 수 있다");

        // 블록2
        {
            let s2 = s1;
            println!("{}", s2);
        }
        // s2의 값은 여기서 파기
        println!("{}", s3);
    }
    // s3의 값은 여기서 파기
    println!("끝");
}