struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
} // User 구조체

// 필드가 없는 유사 유닛 구조체
// 구조체 데이터의 소유권(Ownership)
// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someonename123"),
        active: true,
        sign_in_count: 1,
    }; // User 의 인스턴스 생성
}
