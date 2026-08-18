pub fn slugify(name: &str) -> String {
    let base: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();

    let base: Vec<&str> = base.split('-').filter(|s| !s.is_empty()).collect();
    let base = base.join("-");

    let suffix: u32 = rand::random_range(1000..10000);

    format!("{base}-{suffix}")
}
