pub fn b(text: impl AsRef<str>) -> String {
    format!("<strong>{}</strong>", text.as_ref())
}

pub fn em(text: impl AsRef<str>) -> String {
    format!("<em>{}</em>", text.as_ref())
}

pub fn var(text: impl AsRef<str>) -> String {
    format!("<strong>{}</strong>", text.as_ref())
}

pub fn val<T: std::fmt::Display>(x: T) -> String {
    format!("<strong>{}</strong>", x)
}