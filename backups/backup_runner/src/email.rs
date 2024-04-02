use crate::config::EmailSettings;
use anyhow::Result;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use log::{error, info};

pub struct Email<'a> {
    subject: String,
    content: String,
    email_settings: &'a EmailSettings,
}

impl<'a> Email<'a> {
    pub fn new(email_settings: &'a EmailSettings) -> Self {
        Self {
            email_settings,
            subject: format!("No subject (email notification from {})", get_hostname()).to_string(),
            content: "No body content.".to_string(),
        }
    }

    pub fn subject(mut self, subject: String) -> Self {
        self.subject = subject;

        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = content;

        self
    }

    pub fn send(self) -> Result<()> {
        let email_message = self.to_email()?;
        let smtp_connection = self.get_smtp_transport()?;

        smtp_connection.send(&email_message)?;

        info!(
            "Email message '{}' sent to {}",
            self.subject, self.email_settings.to
        );

        Ok(())
    }

    fn to_email(&self) -> Result<Message> {
        info!("Building email message: {}", self.subject);

        let from_field: Mailbox = self.email_settings.from.parse()?;

        let to_field: Mailbox = self.email_settings.to.parse()?;

        Ok(Message::builder()
            .header(ContentType::TEXT_HTML)
            .from(from_field)
            .to(to_field)
            .subject(&self.subject)
            .body(self.content.to_owned())?)
    }

    fn get_smtp_transport(&self) -> Result<SmtpTransport> {
        info!("Building SMTP connection");

        let creds = Credentials::new(
            self.email_settings.username.to_owned(),
            self.email_settings.password.to_owned(),
        );

        Ok(SmtpTransport::relay(&self.email_settings.host)?
            .credentials(creds)
            .build())
    }
}

fn get_hostname() -> String {
    match gethostname::gethostname().into_string() {
        Ok(s) => s,
        Err(_) => {
            error!("Could not get a valid non-OsString hostname");
            return "unknown-hostname".to_owned();
        }
    }
}
