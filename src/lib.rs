use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

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

    let message = random_message(&answer);
    writeln!(stdout, "{}", message)?;

    Ok(())
}

fn random_message(input: &str) -> String {
    let templates: &[&str] = &[
        "\"{}\" — that file was sent straight to the FBI.",
        "I had a cousin whose entire thing was \"{}\". Never saw him again.",
        "My mom used to say \"{}\" every single morning. She was onto something.",
        "The CIA has \"{}\" on record. Page 47.",
        "Billy Ray Valentine told me the same thing: \"{}\". Right before the margin call.",
        "Winthorpe would have said \"{}\". Probably did, actually.",
        "You know what they put on the Randolph brothers' tombstone? \"{}\".",
        "Last I heard, \"{}\" was trending on the floor of the NYSE.",
        "There's a whole chapter in The Art of the Deal about \"{}\". Look it up.",
        "Duke & Duke tried to patent \"{}\". Got denied.",
    ];

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(42) as usize;

    let template = templates[seed % templates.len()];
    template.replace("{}", input)
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
    use super::*;

    #[test]
    fn trims_non_empty_input() {
        assert_eq!(normalize_input("  Louis Winthorpe  ", "fallback"), "Louis Winthorpe");
    }

    #[test]
    fn falls_back_for_empty_input() {
        assert_eq!(normalize_input("   ", "fallback"), "fallback");
    }

    #[test]
    fn random_message_contains_input() {
        let msg = random_message("hello world");
        assert!(msg.contains("hello world"));
    }

    #[test]
    fn random_message_not_empty() {
        let msg = random_message("test");
        assert!(!msg.is_empty());
    }
}