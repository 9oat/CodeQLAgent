// tools 모듈 - 외부 도구들 (CodeQL 등)
//
// 이 모듈은 에이전트가 사용할 수 있는 도구들을 제공합니다.

// TODO: 하위 모듈 선언
pub mod codeql;      // CodeQL CLI 래퍼
pub mod filesystem;  // 파일 시스템 작업

// Rust 학습 포인트:
// - 모듈을 통해 코드를 논리적 단위로 분리
// - pub으로 외부에 공개할 것과 비공개로 둘 것을 구분
