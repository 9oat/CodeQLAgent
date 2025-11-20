// CodeQL Agent - 소스코드 취약점 분석 에이전트
// 
// 이 프로젝트는 Claude AI와 CodeQL을 결합한 취약점 분석 도구입니다.
// Rust를 배우면서 단계별로 구현해 나가세요!

// TODO: 필요한 모듈들을 가져오기
use clap::Parser;
use anyhow::Result;

// 필요할 때 lib.rs의 모듈을 사용: use codeql_agent::{agent, tools};

#[derive(Parser)]
#[command(name = "codeql_agent", about = "소스코드 취약점 분석 에이전트")]
pub struct Args {
    #[arg(short, long)]
    pub source: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: 1단계 - 기본 CLI 파라미터 파싱
    // - clap을 사용해서 소스코드 경로를 받을 수 있게 만들기
    // - 예: cargo run -- analyze ./my-project
    let args = Args::parse();    
    println!("소스코드 경로: {}", args.source);


    // TODO: 2단계 - 비동기 런타임 설정
    // - tokio를 사용해 async/await 지원
    // - main 함수를 #[tokio::main]으로 변경

    
    // TODO: 3단계 - 에이전트 초기화 및 실행
    // - agent::orchestrator 모듈의 함수 호출
    // - 결과를 받아서 출력
    
    println!("CodeQL Agent - 아직 구현 전입니다!");
    println!("main.rs의 TODO를 확인하고 하나씩 구현해보세요.");
    Ok(())
}
