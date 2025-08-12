use anyhow::{Result, anyhow};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, MultiPart, SinglePart, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use rand::Rng;

#[derive(Debug)]
pub enum CodeType {
    REGISTER,
    RESET,
}

pub fn generate_verification_code() -> String {
    let mut rng = rand::rng();
    let code: u32 = rng.random_range(100000..=999999);
    code.to_string()
}

#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
}

impl EmailConfig {
    pub fn from_env() -> Result<Self> {
        let smtp_server =
            std::env::var("SMTP_SERVER").map_err(|_| anyhow!("SMTP server is not configured"))?;
        let smtp_port = std::env::var("SMTP_PORT")
            .map_err(|_| anyhow!("SMTP port is not configured"))?
            .parse::<u16>()?;
        let smtp_username = std::env::var("SMTP_USERNAME")
            .map_err(|_| anyhow!("SMTP username is not configured"))?;
        let smtp_password = std::env::var("SMTP_PASSWORD")
            .map_err(|_| anyhow!("SMTP password is not configured"))?;
        let from_email =
            std::env::var("FROM_EMAIL").map_err(|_| anyhow!("From email is not configured"))?;

        Ok(Self {
            smtp_server,
            smtp_port,
            smtp_username,
            smtp_password,
            from_email,
        })
    }
}

pub async fn send_verification_email(
    config: &EmailConfig,
    to_email: String,
    code: &str,
    code_type: &CodeType,
) -> Result<()> {
    tracing::info!("Email config is:\n{config:?}");

    let expire_minutes = 10;

    let to_address: Mailbox = to_email
        .parse()
        .map_err(|_| anyhow!("To email is invalid"))?;

    let from_address: Mailbox = config
        .from_email
        .parse()
        .map_err(|_| anyhow!("From email is invalid"))?;

    let subject = match code_type {
        CodeType::REGISTER => "Register Code",
        CodeType::RESET => "Reset Password Code",
    };

    let html_body = format!(
        r##"
        <html>
            <body style="font-family: Arial, sans-serif; background-color: #f6f6f6; padding: 20px;">
                <table width="100%" cellpadding="0" cellspacing="0" style="max-width: 600px; margin: auto; background: white; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 8px rgba(0,0,0,0.05);">
                    <tr style="background-color: #004080;">
                        <td style="padding: 15px; text-align: center;">
                            <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" fill="none"><mask id="mask0_18965_4956" width="50" height="50" x="0" y="0" maskUnits="userSpaceOnUse" style="mask-type:alpha"><path fill="#D9D9D9" d="M0 16C0 7.163 7.163 0 16 0h18c8.837 0 16 7.163 16 16v18c0 8.837-7.163 16-16 16H16C7.163 50 0 42.837 0 34V16Z"/></mask><g mask="url(#mask0_18965_4956)"><path fill="#FFC550" d="M0 0h50v50H0z"/><mask id="mask1_18965_4956" width="36" height="28" x="7" y="11" maskUnits="userSpaceOnUse" style="mask-type:alpha"><rect width="35.877" height="26.347" x="7.06" y="11.83" fill="#fff" rx="5.389"/></mask><g mask="url(#mask1_18965_4956)"><rect width="35.877" height="26.347" x="7.06" y="11.83" fill="#fff" rx="5.389"/><g filter="url(#filter0_f_18965_4956)" opacity=".5"><path fill="gray" d="m7.06 16.398 14.892 14.454a4.375 4.375 0 0 0 6.093 0l14.892-14.454H7.06Z"/></g><path fill="#FF5B10" d="M37.547 11.83a5.39 5.39 0 0 1 5 3.374c.175.435.238.912.154 1.374-.163.896-.63 1.761-1.457 2.446l-13.57 11.247a4.191 4.191 0 0 1-5.35 0L8.754 19.024c-.827-.685-1.294-1.55-1.458-2.446a2.49 2.49 0 0 1 .155-1.374 5.39 5.39 0 0 1 4.998-3.374h25.098Z"/></g></g><defs><filter id="filter0_f_18965_4956" width="40.829" height="20.642" x="4.584" y="13.922" color-interpolation-filters="sRGB" filterUnits="userSpaceOnUse"><feFlood flood-opacity="0" result="BackgroundImageFix"/><feBlend in="SourceGraphic" in2="BackgroundImageFix" result="shape"/><feGaussianBlur result="effect1_foregroundBlur_18965_4956" stdDeviation="1.238"/></filter></defs></svg>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px; font-size: 16px; color: #333;">
                            <p>Dear User,</p>
                            <p>Thank you for registering with our service! To complete your registration, please use the verification code below:</p>                            <p>Please confirm your verification code:</p>
                            <p style="text-align: center; font-size: 24px; font-weight: bold; letter-spacing: 4px; margin: 30px 0; color: #004080;">
                                {code}
                            </p>
                            <p>This code will expire in <b>{expire_minutes} minutes</b>. Please do not share it with anyone.</p>
                            <p>If you did not request this code, please ignore this email.</p>
                            <br>
                            <p>Best regards,<br><b>Adam Daniel</b><br>Customer Support Team</p>
                        </td>
                    </tr>
                    <tr style="background-color: #f1f1f1; text-align: center; font-size: 12px; color: #888;">
                        <td style="padding: 10px;">
                            © 2025 Adam. All rights reserved.  
                            <br>Longhua Street, Shenzhen, GongDong, China
                        </td>
                    </tr>
                </table>
            </body>
        </html>
    "##
    );

    let text_body = format!(
        r#"
Dear User,

Thank you for registering with our service! We are excited to have you on board.

Please confirm your verification code:

{code}

This code will expire in {expire_minutes} minutes. 
Please do not share it with anyone.

If you did not sign up for our service, please ignore this email.

Best regards,
Adam Daniel
Customer Support Team
    "#
    );

    let email = Message::builder()
        .from(from_address)
        .to(to_address)
        .subject(subject)
        // .header(ContentType::TEXT_HTML)
        // .body(html_body.to_string())
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                        .body(text_body.to_string()),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(html_body),
                ),
        )
        .map_err(|_| anyhow!("Create email failed"))?;

    let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

    let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_server)
        .map_err(|_| anyhow!("Create SMTP middleware failed"))?
        .port(config.smtp_port)
        .credentials(creds)
        .build();

    tracing::debug!("{:?}", email.envelope());

    transport
        .send(email)
        .await
        .map_err(|error| anyhow!("Send email to {to_email} error {error}"))?;

    tracing::info!("success to send to {to_email}");

    Ok(())
}
