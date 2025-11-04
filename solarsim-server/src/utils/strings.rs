/// Capitalizes "snake_case" words (e.g. snake_case -> Snake Case)
pub fn capitalize(str: impl Into<String>) -> String {
    str.into()
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or(String::default(), |char| {
                char.to_uppercase().collect::<String>() + chars.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}
