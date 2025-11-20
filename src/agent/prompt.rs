// prompt.rs - Claude에게 보낼 프롬프트 템플릿
//
// 이 파일은 에이전트가 Claude와 대화할 때 사용할 시스템 프롬프트를 정의합니다.

// Rust 학습 포인트:
// - `const`로 상수 정의
// - 문자열 리터럴 `&str` vs 소유 문자열 `String`
// - 여러 줄 문자열은 `r#"...내용..."#` 사용 가능

// TODO: 시스템 프롬프트 작성
// pub const SYSTEM_PROMPT: &str = r#"
// 당신은 소스코드 보안 전문가입니다.
// 주어진 소스코드의 취약점을 분석하고 보고서를 작성합니다.
// 
// 사용 가능한 도구:
// 1. CodeQL - 코드의 데이터 흐름과 제어 흐름을 분석
//    - SQL injection 탐지
//    - XSS 취약점 탐지
//    - 경로 순회 취약점 탐지
//    등등...
// 
// 분석 절차:
// 1. 코드베이스 개요 파악
// 2. 의심스러운 패턴 식별
// 3. CodeQL 쿼리 선택 및 실행
// 4. 결과 해석 및 위험도 평가
// 5. 수정 방법 제안
// "#;

// TODO: 도구 사용 지시 프롬프트
// pub fn get_codeql_instruction(query_type: &str) -> String {
//     // CodeQL을 어떻게 사용할지 알려주는 프롬프트
//     // 예: "다음 쿼리를 실행해주세요: sql-injection.ql"
// }

pub fn placeholder() {
    println!("prompt.rs - 여기에 프롬프트 템플릿을 작성하세요!");
}
