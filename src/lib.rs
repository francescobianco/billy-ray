use std::io::{self, Write};

/// Run the Billy Ray interactive dialogue.
pub fn run() -> io::Result<()> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    writeln!(stdout, "Hello, my name is Openapi. Who are you?")?;
    write!(stdout, "> ")?;
    stdout.flush()?;

    let mut name = String::new();
    stdin.read_line(&mut name)?;
    let name = normalize_input(&name, "stranger");

    writeln!(
        stdout,
        "Well, Billy Ray, what happened to that loser named {}?",
        name
    )?;
    write!(stdout, "> ")?;
    stdout.flush()?;

    let mut answer = String::new();
    stdin.read_line(&mut answer)?;
    let answer = normalize_input(&answer, "silence");

    writeln!(stdout, "Alexa, per favore... {}", answer)?;
    Ok(())
}

fn normalize_input(input: &str, fallback: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_input;

    #[test]
    fn trims_non_empty_input() {
        assert_eq!(normalize_input("  Louis Winthorpe  ", "fallback"), "Louis Winthorpe");
    }

    #[test]
    fn falls_back_for_empty_input() {
        assert_eq!(normalize_input("   ", "fallback"), "fallback");
    }
}
