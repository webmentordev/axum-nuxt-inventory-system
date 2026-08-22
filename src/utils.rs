pub fn slugify(name: &str, random: bool) -> String {
    let normalized: String = name
        .chars()
        .map(|c| match c {
            '₀' => '0',
            '₁' => '1',
            '₂' => '2',
            '₃' => '3',
            '₄' => '4',
            '₅' => '5',
            '₆' => '6',
            '₇' => '7',
            '₈' => '8',
            '₉' => '9',
            '⁰' => '0',
            '¹' => '1',
            '²' => '2',
            '³' => '3',
            '⁴' => '4',
            '⁵' => '5',
            '⁶' => '6',
            '⁷' => '7',
            '⁸' => '8',
            '⁹' => '9',
            '–' | '—' | '−' => '-',
            other => other,
        })
        .collect();

    let base: String = normalized
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();

    let base: Vec<&str> = base.split('-').filter(|s| !s.is_empty()).collect();
    let base = base.join("-");

    if random {
        let suffix: u32 = rand::random_range(1000..10000);
        return format!("{base}-{suffix}");
    }

    base
}
