use std::io::Write;
use anyhow::Result;

pub fn find_matches(
    content: &str, 
    pattern: &str, 
    mut writer: impl Write
) -> Result<()> {
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
}
