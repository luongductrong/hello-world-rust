use std::fmt::Display;

pub fn run() {
    let e_noti = EmailNotification {
        to_address: "gay@bi.ex".to_string(),
        subject: "KHÔNG CÓ GÌ HEHE".to_string(),
        body: "Hỏi thử rảnh không á mà".to_string(),
    };

    let sms_noti = SmsNotification::from(("0912345678", "Mã OTP là 1234"));

    println!("{e_noti}");
    println!("{}", e_noti.send());

    println!("{sms_noti}");
    println!("{}", sms_noti.send());
}

trait Maskable {
    fn mask(&self) -> String;
}

trait Notification {
    fn format_message(&self) -> String;
    fn send(&self) -> String {
        format!("[ĐÃ GỬI] {}", self.format_message())
    }
}

struct EmailNotification {
    to_address: String,
    subject: String,
    body: String,
}

struct SmsNotification {
    phone_number: String,
    message: String,
}

impl Maskable for String {
    fn mask(&self) -> String {
        if self.len() <= 4 {
            "****".to_string()
        } else {
            format!("{}****", &self[..3])
        }
    }
}

impl Notification for EmailNotification {
    fn format_message(&self) -> String {
        format!(
            "Email đến {}: [{}] {}",
            self.to_address, self.subject, self.body
        )
    }
}

impl Notification for SmsNotification {
    fn format_message(&self) -> String {
        format!("SMS đến {}: {}", self.phone_number.mask(), self.message)
    }
}

impl Display for EmailNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_message())
    }
}

impl Display for SmsNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_message())
    }
}

impl From<(&str, &str)> for SmsNotification {
    fn from((phone_number, message): (&str, &str)) -> Self {
        Self {
            phone_number: phone_number.to_string(),
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_notification() {
        let e_noti = EmailNotification {
            to_address: "gay@bi.ex".to_string(),
            subject: "KHÔNG CÓ GÌ HEHE".to_string(),
            body: "Hỏi thử rảnh không á mà".to_string(),
        };

        assert_eq!(
            e_noti.send(),
            "[ĐÃ GỬI] Email đến gay@bi.ex: [KHÔNG CÓ GÌ HEHE] Hỏi thử rảnh không á mà"
        );
    }

    #[test]
    fn test_sms_from() {
        let sms_noti = SmsNotification::from(("0123456789", "Ê chiều nhậu không?"));

        assert_eq!(sms_noti.message, "Ê chiều nhậu không?");
        assert_eq!(sms_noti.phone_number, "0123456789");
    }

    #[test]
    fn test_markable() {
        assert_eq!("0123455678".to_string().mask(), "012****".to_string());
        assert_eq!("0122".to_string().mask(), "****".to_string());
    }
}
