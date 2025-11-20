// codeql.rs

use std::process::Command;
use crate::error::{AppError, Result};
use std::path::PathBuf;
use std::fs;

pub struct CodeQLRunner {
    db_path: String
}

impl CodeQLRunner {
    pub fn new(db_path: impl Into<String>) -> Result<Self> {
        Command::new("codeql")
            .arg("--version")
            .output()?;
        
        Ok(CodeQLRunner {
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

#[tokio::test]
async fn test_run_cpp_variable_query() {
    let runner = CodeQLRunner::new("/home/goat/aaa/curl/curl.ql")
        .expect("CodeQL CLI가 설치되어 있어야 합니다");
    
    let query = r#"
    import cpp

    from VariableAccess a, Variable v
    where
    a.getFile().getRelativePath() = "src/var.c" and
    a.getLocation().getStartLine() = 362 and
    a.getTarget() = v and
    v.getName() = "p"
    select v,
    v.getType(),
    v.getLocation().getStartLine(),
    v.getLocation().getStartColumn()
    "#;
        
    let csv_result = runner.run_query(query).await
        .expect("쿼리 실행 실패");

    println!("CSV:\n{}", csv_result);
    
    assert!(!csv_result.is_empty(), "CSV 결과가 비어있으면 안됨");
    assert!(csv_result.contains("p"), "변수 'p'가 결과에 포함되어야 함");
}

pub struct CodeQLAnalyzer {
    runner: CodeQLRunner,
}

impl CodeQLAnalyzer{
    pub fn new(runner: CodeQLRunner) -> Self {
        CodeQLAnalyzer {
            runner,
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
        select v,
        v.getType(),
        v.getLocation().getStartLine(),
        v.getLocation().getStartColumn()
        "#, filename, line, varname);
        self.runner.run_query(query).await
    }
}