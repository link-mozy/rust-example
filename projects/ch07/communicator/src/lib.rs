pub mod client;
pub mod network;

// # 비공개 규칙 (Privacy Rules)
// 1. 만일 어떤 아이템이 공개라면, 이는 부모 모듈의 어디에서건 접근 가능합니다.
// 2. 만일 어떤 아이템이 비공개라면, 같은 파일 내에 있는 부모 모듈 및 이 부모의 자식 모듈에서만 접근 가능합니다.

// # super를 사용하여 부모 모듈에 접근하기
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         super::client::connect()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}