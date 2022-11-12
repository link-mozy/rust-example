mod outermost {
    pub fn middle_function () {}

    fn middle_secret_function () {}

    mod inside {
        pub fn inner_function () {}

        fn secret_function () {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
// 여기 이 에러들을 고치기 위해 코드를 수정하는것에 관한 몇 가지 제안이 있습니다. 각각을 시도해보기 전에, 이 시도가 에러를 고칠지 그렇지 않을지 추측해 보고, 컴파일을 해서 여러분이 맞췄는지 그렇지 않은지 확인하고, 왜 그랬는지 이해하기 위해 비공개 규칙을 이용해보세요.
//
// inside 모듈이 공개라면 어떨까요?
// outermost가 공개고 inside가 비공개면 어떨까요?
// inner_function의 내부에서 ::outermost::middle_secret_function()을 호출한다면 어떨까요? (시작 부분의 콜론 두개는 루트 모듈로부터 시작하여 모듈을 참조하고 싶음을 나타냅니다)
// 자유롭게 더 많은 실험을 설계하고 시도해 보세요!
//
// 다음으로, use 키워드를 사용하여 아이템을 스코프 내로 가져오는 것에 대해 이야기해 봅시다.