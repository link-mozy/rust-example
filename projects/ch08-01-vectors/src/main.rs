// # 벡터
// 메모리상에서 서로 이웃하도록 모든 값을 집어 넣어 단일 데이터 구조 안에 하나 이상의 값을 저장하도록 한다.
// 벡터는 같은 타입만 저장 가능하다!
// -> Stack 과 비슷해보인다.

fn main() {
    // # 새 백터 만들기
    let mut v: Vec<i32> = Vec::new();
    // 어떠한 값도 처음에 넣어 주지 않았기때문에 데이터 타입을 명시 해줬다.

    let v2 = vec![1, 2, 3];
    // vec! 매크로를 사용하여 Vec<i32>를 생성

    // # 백터 갱신하기
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // # 벡터를 드롭하는 것은 벡터의 요소들을 드롭시킵니다
    {
        let v3 = vec![1, 2, 3, 4];
        // v3 를 가지고 뭔가 한다.
    } // <- v3가 스코프 밖으로 벗어났고, 여기서 해제된다.

    // # 벡터의 요소들 읽기
    let v4 = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v4[2];
    // let third: Option<&i32> = v4.get(2);

    // let does_not_exist = &v[100];
    // println!("does_not_exist is {}", does_not_exist);
    // thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 100', src/main.rs:32:27
    // 벡터의 끝을 넘어서는 요소에 접근하는 시도를 하면 프로그램이 죽게끔 하는 치명적 에러를 발생하도록 하기를 고려하는 경우 가장 좋습니다.
    // let does_not_exist = v.get(100);
    // println!("does_not_exist is {:?}", does_not_exist);
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // `get` 함수에 벡터 범위를 벗어난 인덱스가 주어졌을 때는 패닉 없이 None이 반환됩니다.
    // 이런 편이 오타 때문에 프로그램이 죽는 것 보다는 더 사용자 친화적이겠죠!

    // # 유효하지 않는 참조자
    let mut v5 = vec![1, 2, 3, 4, 5];
    let first = &v5[0];
    v5.push(6);

    // # 벡터 내의 값들에 대한 반복처리
    let v6 = vec![100, 32, 57];
    for i in &v6 {
        println!("{}", i);
    }

    let mut v7 = vec![100, 32, 57];
    for i in &mut v7 {
        println!("{}", *i);
        *i += 50;
        // 역참조 연산자 (*)를 사용하여 값을 얻어야 합니다.
        println!("{}", *i);
    }

    // # 열거형을 사용하여 여러 타입을 저장하기
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // enum 을 정의하여 벡터 내에 다른 타입의 데이터를 담을 수 있도록 하기
    // push에 더해서, pop 메소드는 제일 마지막 요소를 반환하고 지워준다.
}
