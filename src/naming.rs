pub fn key(parts: &[&str]) -> String {
    parts.join("_")
}

pub fn slug(s: &str) -> String {
    s.to_lowercase()
        .replace('ł', "l")
        .replace('ó', "o")
        .replace('ś', "s")
        .replace('ż', "z")
        .replace('ź', "z")
        .replace('ć', "c")
        .replace('ń', "n")
        .replace('ą', "a")
        .replace('ę', "e")
        .replace(' ', "_")
}