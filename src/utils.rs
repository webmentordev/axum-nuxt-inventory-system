use uuid::Uuid;

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

pub fn generate_sku(name: &str) -> String {
    let clean_name: String = name
        .to_uppercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();
    let prefix: String = clean_name
        .split_whitespace()
        .take(2)
        .map(|word| word.chars().take(2).collect::<String>())
        .collect::<Vec<String>>()
        .join("-");
    let suffix: u32 = rand::random_range(100..=999);
    format!("{}-{}", prefix, suffix)
}

pub fn generate_order_number() -> String {
    let number: u32 = rand::random_range(1_000_000..10_000_000);
    number.to_string()
}

pub async fn log_audit(
    db: &sqlx::PgPool,
    user_id: Option<Uuid>,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    status: &str,
    details: Option<serde_json::Value>,
) {
    let _ = sqlx::query!(
        r#"INSERT INTO audit_logs (user_id, action, entity_type, entity_id, status, details)
           VALUES ($1, $2, $3, $4, $5, $6)"#,
        user_id,
        action,
        entity_type,
        entity_id,
        status,
        details
    )
    .execute(db)
    .await;
}
