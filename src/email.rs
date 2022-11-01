use mail_parser::Message;

pub struct Email {
    pub timestamp: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub cc: Option<String>,
    pub bcc: Option<String>,
    pub subject: String,
    pub text_body: String,
    pub html_body: String,
    pub gmail_labels: Option<String>
}

impl Email {
    pub fn from(msg: &Message) -> Email {
        return Email {
            timestamp: msg.get_date().map(|it| it.to_rfc822()),
            from: msg.get_from().as_text_ref().map(|it| String::from(it)),
            to: msg.get_to().as_text_ref().map(|it| String::from(it)),
            cc: msg.get_cc().as_text_ref().map(|it| String::from(it)),
            bcc: msg.get_bcc().as_text_ref().map(|it| String::from(it)),
            subject: String::from(msg.get_subject().unwrap_or("")),
            text_body: msg.get_text_bodies()
                .map(|it| {
                    it.get_text_contents().unwrap_or("")
                })
                .collect::<String>(),
            html_body: msg.get_html_bodies()
                .map(|it| {
                    it.get_text_contents().unwrap_or("")
                })
                .collect::<String>(),
            gmail_labels: msg.get_header("X-Gmail-Labels")
                .map(|it| {
                    String::from(it.as_text_ref().unwrap_or(""))
                })
        }
    }
}
