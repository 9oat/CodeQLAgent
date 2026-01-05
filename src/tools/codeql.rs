// codeql.rs

use std::process::Command;
use crate::error::{AppError, Result};
use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::utils::utils::parse_csv;
use crate::tools::filesystem::FileSystem;

pub struct CodeQLRunner {
    db_path: String,
    src_path: String,
}

impl CodeQLRunner {
    pub fn new(src_path: impl Into<String>, db_path: impl Into<String>) -> Result<Self> {
        Command::new("codeql")
            .arg("--version")
            .output()?;
        
        Ok(CodeQLRunner {
            src_path: src_path.into(),
            db_path: db_path.into(),
        })
    }
    
    pub async fn create_database(
        &self, 
        source_path: &str,
        language: &str
    ) -> Result<()> {

        Command::new("codeql")
            .arg("database")
            .arg("create")
            .arg(format!("--language={}", language))
            .arg(format!("--source-root={}", source_path))
            .arg(&self.db_path)
            .output()?;
        Ok(())
    }

    pub async fn run_query(
        &self,
        query_string: &str
    ) -> Result<String> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let work_dir = PathBuf::from("tmp").join(format!("query_{}", timestamp));
        fs::create_dir_all(&work_dir)?;
        
        let language = self.detect_language(query_string);
        
        let qlpack_path = work_dir.join("qlpack.yml");
        let qlpack_content = format!(r#"name: tmpql-{}
version: 0.0.1
libraryPathDependencies:
  - codeql/{}-all
"#, timestamp, language);
        fs::write(&qlpack_path, qlpack_content)?;

        let query_path = work_dir.join("query.ql");
        let bqrs_path = work_dir.join("result.bqrs");
        let csv_path = work_dir.join("result.csv");
        
        fs::write(&query_path, query_string)?;
        
        let output = Command::new("codeql")
            .arg("query")
            .arg("run")
            .arg(&query_path)
            .arg(format!("--database={}", &self.db_path))
            .arg(format!("--output={}", bqrs_path.display()))
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let _ = fs::remove_dir_all(&work_dir);
            return Err(AppError::CodeQLError(stderr));
        }
        
        let output = Command::new("codeql")
            .arg("bqrs")
            .arg("decode")
            .arg(&bqrs_path)
            .arg("--format=csv")
            .arg(format!("--output={}", csv_path.display()))
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let _ = fs::remove_dir_all(&work_dir);
            return Err(AppError::CodeQLError(stderr));
        }
        
        let csv_content = fs::read_to_string(&csv_path)?;
        
        let _ = fs::remove_dir_all(&work_dir);
        Ok(csv_content)
    }
    
    fn detect_language(&self, query_string: &str) -> &str {
        if query_string.contains("import cpp") {
            "cpp"
        } else if query_string.contains("import python") {
            "python"
        } else if query_string.contains("import java") {
            "java"
        } else if query_string.contains("import javascript") {
            "javascript"
        } else if query_string.contains("import csharp") {
            "csharp"
        } else {
            "cpp"
        }
    }
}

// #[tokio::test]
// async fn test_run_cpp_variable_query() {
//     let runner = CodeQLRunner::new("/home/goat/aaa/curl", "/home/goat/aaa/curl/curl.ql")
//         .expect("CodeQL CLI가 설치되어 있어야 합니다");
    
//     let query = r#"
//     import cpp

//     from VariableAccess a, Variable v
//     where
//     a.getFile().getRelativePath() = "src/var.c" and
//     a.getLocation().getStartLine() = 362 and
//     a.getTarget() = v and
//     v.getName() = "p"
//     select v as variable,
//     v.getType() as type,
//     v.getLocation().getStartLine() as line,
//     v.getLocation().getStartColumn() as column
//     "#;
        
//     let csv_result = runner.run_query(&query).await
//         .expect("쿼리 실행 실패");

//     println!("CSV:\n{}", csv_result);
    
//     assert!(!csv_result.is_empty(), "CSV 결과가 비어있으면 안됨");
//     assert!(csv_result.contains("p"), "변수 'p'가 결과에 포함되어야 함");
// }

// 소스코드와 라인 정보를 반환하는 녀석

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceInfoParse {
    pub filename: String,
    pub startline: u32,
    pub endline: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceInfoResult {
    pub filename: String,
    pub line: u32,
    pub code: String
}

pub struct CodeQLAnalyzer {
    runner: CodeQLRunner,
    fs: FileSystem,
}

#[derive(Debug, Serialize, Deserialize)]
struct FunctionInfoParse {
    qualified_name: String,
    filename: String,
    startline: u32,
    endline: u32,
    is_virtual: String,  // "true" 또는 "false" 문자열로 옴
}

#[derive(Debug, Serialize, Deserialize)]
struct OverrideInfoParse {
    qualified_name: String,
    filename: String,
    startline: u32,
    endline: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct FunctionInfoResult {
    qualified_name: String,
    filename: String,
    line: u32,
    code: String,
    is_virtual: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<Vec<FunctionInfoResult>>,
}

impl CodeQLAnalyzer{
    pub fn new(runner: CodeQLRunner) -> Self {
        CodeQLAnalyzer {
            runner,
            fs: FileSystem::new(),
        }
    }

    pub async fn find_var_definitions(&self, filename: &str, line: u32, varname: &str) -> Result<String> {
        let query = format!(r#"
        import cpp

        from VariableAccess a, Variable v
        where
        a.getFile().getRelativePath() = "{}" and
        a.getLocation().getStartLine() = {} and
        a.getTarget() = v and
        v.getName() = "{}"
        select 
        v.getFile().getRelativePath() as filename,
        v.getLocation().getStartLine() as startline,
        v.getLocation().getEndLine() as endline
        "#, filename, line, varname);
        let csv_result = self.runner.run_query(&query).await?;
        let parsed: Vec<SourceInfoParse> = parse_csv(&csv_result)?;
        
        if parsed.is_empty() {
            return Err(AppError::CodeQLError("No results".to_string()));
        }else if parsed.len() > 1 {
            return Err(AppError::CodeQLError("Multiple results".to_string()));
        }

        let source_info = &parsed[0];
        let filepath = PathBuf::from(&self.runner.src_path).join(&source_info.filename);
        let source_code = self.fs.read_file_lines(&filepath, source_info.startline, source_info.endline)?;
        let result = SourceInfoResult {
            filename: filepath.display().to_string(),
            line: source_info.startline,
            code: source_code.join("\n"),
        };
        Ok(serde_json::to_string_pretty(&result)?)
    }

    pub async fn find_function_implementation(&self, filename: &str, line: u32, funcname: &str) -> Result<String> {
        let query = format!(r#"
        import cpp

        from FunctionCall fc, Function target, string is_virtual
        where
        fc.getFile().getRelativePath() = "{}" and
        fc.getLocation().getStartLine() = {} and
        target = fc.getTarget() and
        (
            target.getName() = "{}"
            or target.getQualifiedName().matches("%{}%")
        ) and
        (
            (target instanceof VirtualFunction and is_virtual = "true")
            or
            (not target instanceof VirtualFunction and is_virtual = "false")
        )
        select 
        target.getQualifiedName() as qualified_name,
        target.getFile().getRelativePath() as filename,
        target.getLocation().getStartLine() as startline,
        target.getLocation().getEndLine() as endline,
        is_virtual
        "#, filename, line, funcname, funcname);
        
        let csv_result = self.runner.run_query(&query).await?;
        let parsed: Vec<FunctionInfoParse> = parse_csv(&csv_result)?;
        
        if parsed.is_empty() {
            return Err(AppError::CodeQLError("No function call found at specified location".to_string()));
        }

        let mut results = Vec::new();
        
        for func_info in parsed {
            let filepath = PathBuf::from(&self.runner.src_path).join(&func_info.filename);
            let source_code = self.fs.read_file_lines(&filepath, func_info.startline, func_info.endline)?;
            
            let is_virtual = func_info.is_virtual == "true";
            
            let mut result = FunctionInfoResult {
                qualified_name: func_info.qualified_name.clone(),
                filename: filepath.display().to_string(),
                line: func_info.startline,
                code: source_code.join("\n"),
                is_virtual,
                overrides: None,
            };

            // 가상 함수면 오버라이드 찾기
            if is_virtual {
                result.overrides = Some(self.find_function_overrides(&func_info.qualified_name).await?);
            }

            results.push(result);
        }

        Ok(serde_json::to_string_pretty(&results)?)
    }

    async fn find_function_overrides(&self, qualified_name: &str) -> Result<Vec<FunctionInfoResult>> {
        let query = format!(r#"
        import cpp

        from VirtualFunction base, Function override
        where
        base.getQualifiedName() = "{}" and
        override = base.getAnOverridingFunction()
        select 
        override.getQualifiedName() as qualified_name,
        override.getFile().getRelativePath() as filename,
        override.getLocation().getStartLine() as startline,
        override.getLocation().getEndLine() as endline
        "#, qualified_name);

        let csv_result = self.runner.run_query(&query).await?;
        let parsed: Vec<OverrideInfoParse> = parse_csv(&csv_result)?;

        let mut overrides = Vec::new();
        for override_info in parsed {
            let filepath = PathBuf::from(&self.runner.src_path).join(&override_info.filename);
            let source_code = self.fs.read_file_lines(&filepath, override_info.startline, override_info.endline)?;
            
            overrides.push(FunctionInfoResult {
                qualified_name: override_info.qualified_name,
                filename: filepath.display().to_string(),
                line: override_info.startline,
                code: source_code.join("\n"),
                is_virtual: true,
                overrides: None,
            });
        }

        Ok(overrides)
    }
}

// #[tokio::test]
// async fn test_find_var_definitions() {
//     let runner = CodeQLRunner::new("/home/goat/aaa/curl", "/home/goat/aaa/curl/curl.ql")
//         .expect("CodeQL CLI가 설치되어 있어야 합니다");
//     let analyzer = CodeQLAnalyzer::new(runner);
//     let result = analyzer.find_var_definitions("src/var.c", 362, "p").await;
//     println!("Result: {:#?}", result);
//     assert!(result.is_ok());
// }

#[tokio::test]
async fn test_find_function_implementation(){
    let runner = CodeQLRunner::new("/home/goat/aaa/curl", "/home/goat/aaa/curl/curl.ql")
        .expect("CodeQL CLI가 설치되어 있어야 합니다");
    let analyzer = CodeQLAnalyzer::new(runner);
    let result = analyzer.find_function_implementation("src/var.c", 465, "file2memory_range").await;
    println!("Result: {:#?}", result);
    assert!(result.is_ok());
}