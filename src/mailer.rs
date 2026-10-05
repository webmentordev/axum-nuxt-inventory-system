use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};

#[derive(Clone)]
pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
    pub frontend_url: String,
}

impl Mailer {
    pub fn from_env() -> anyhow::Result<Self> {
        let host = std::env::var("SMTP_HOST")?;
        let user = std::env::var("SMTP_USER")?;
        let pass = std::env::var("SMTP_PASS")?;
        let from = std::env::var("MAIL_FROM")?;
        let frontend_url = std::env::var("FRONTEND_URL")?
            .trim_end_matches('/')
            .to_string();

        let port: u16 = std::env::var("SMTP_PORT")?.parse()?;
        let secure = std::env::var("SMTP_SECURITY").unwrap_or_else(|_| "starttls".into());

        let builder = match secure.as_str() {
            "tls" => AsyncSmtpTransport::<Tokio1Executor>::relay(&host)?,
            "starttls" => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)?,
            _ => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&host),
        };

        let transport = builder
            .port(port)
            .credentials(Credentials::new(user, pass))
            .build();

        Ok(Self {
            transport,
            from,
            frontend_url,
        })
    }

    pub async fn send(&self, to: &str, subject: &str, body: String) -> anyhow::Result<()> {
        let message = Message::builder()
            .from(self.from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        self.transport.send(message).await?;
        Ok(())
    }
}
