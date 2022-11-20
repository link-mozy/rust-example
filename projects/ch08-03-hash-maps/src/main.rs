use std::collections::HashMap;

fn main() {
    // # 새로운 해쉬맵 생성하기
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 벡터와 비슷하게 해쉬맵도 동질적이다.
    // 모든 키는 같은 타입이어야 하고, 모든 값도 같은 타입이어야 한다.

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
    // 팀의 리스트와 점수의 리스트로부터 해쉬맵 생성하기
    // 타입 명시 HashMap<_, _> 이 필요한데 이는 collect 가 다른 많은 데이터 구조로 바뀔 수 있고,
    // 우리가 어떤 타입인지 모르기 때문에 _를 사용함.
    // 키와 값으 타입에 대한 타입 파라미터에 대해서는 밑줄을 쓸 수 있으며 러스트는 벡터에 담긴 데이터의 타입에 기초하여 해쉬에 담길 타입을 추론할 수 있다.

    // # 해쉬맵과 소유권
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name과 field_value은 이 지점부터 유효하지 않는다!
    // 이들을 이요하는 시도를 해보고 어떤 컴파일러 에러가 나오는지 보자.

    println!("map is {:?}", map);
    // println!("field_name is {}", field_name);
    // value borrowed here after move
    // this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

    // # 해쉬맵 내의 값 접근하기
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score is {:?}", score); // score is Some(10)
    // 결과값은 Some으로 감싸져 있는데 왜냐하면 get이 Option<&V>를 반환하기 때문입니다;

    let mut scores2 = HashMap::new();

    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Yellow"), 50);

    for (key, value) in scores2 {
        println!("{} : {}", key, value);
    }
    // println result :
    // Yellow : 50
    // Blue : 10
    // 여기서도 추측건데 stack 구조로 되어 lifo 형태인것을 확인

    // # 값을 덮어쓰기
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);
    println!("{:?}", scores2);

    // # 키에 할당된 값이 없을 경우에만 삽입하기
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);

    scores3.entry(String::from("Yellow")).or_insert(50);
    scores3.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores3);

    // # 예전 값을 기초로 값을 갱신하기
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    println!("** 단어와 횟수를 저장하는 해쉬맵을 사용하여 단어의 등장 횟수 세기 **");
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
