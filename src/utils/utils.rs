use serde::de::DeserializeOwned;
use crate::error::Result;

pub fn parse_csv<T: DeserializeOwned>(csv_data: &str) -> Result<Vec<T>> {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut results = Vec::new();
    
    for result in reader.deserialize() {
        let record: T = result?;
        results.push(record);
    }
    
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_csv() {
        let csv_data = r#"variable,type,line,column
p,tool_var *,356,20"#;
        let result = parse_csv::<(String, String, u32, u32)>(csv_data).unwrap();
        assert_eq!(result, vec![("p".to_string(), "tool_var *".to_string(), 356, 20)]);
    }
}