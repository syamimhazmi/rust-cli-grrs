use std::io::Write;
use anyhow::{bail, Result};

pub fn find_matches(
    content: &str, 
    pattern: &str, 
    mut writer: impl Write
) -> Result<()> {
    if pattern.is_empty() {
        bail!("unexpected argument '' found");
    }

    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_matches() {
        let content = "Hello World\nRust is awesome\nHello Rust\n";
        let pattern = "Hello";
        let mut result = Vec::new();

        find_matches(content, pattern, &mut result).unwrap();

        assert_eq!(
            String::from_utf8(result).unwrap(),
            "Hello World\nHello Rust\n",
        );
    }
    
    #[test]
    fn test_empty_pattern() {
        let content = "Hello World\nRust is awesome\nHello Rust\n";
        let pattern = "";
        let mut result = Vec::new();
        
        let err = find_matches(content, pattern, &mut result);

        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err().to_string(), 
            "unexpected argument '' found",
        );
    }
}
