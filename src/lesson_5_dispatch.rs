pub fn run() {
    let mut credit = CreditCard {
        card_number: "4508 7500 1574 1019".to_string(),
        holder_name: "Tao".to_string(),
    };
    let mut momo = MomoWallet {
        phone: "0123456789".to_string(),
        balance: 5_000_000,
    };
    let mut crypto = CryptoWallet {
        wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    };

    // rảnh rỗi gọi cả 3 luôn
    run_pay_single(&mut credit, 70_000_000);
    run_pay_single(&mut momo, 10_000);
    run_pay_single(&mut crypto, u64::MAX);

    println!("---------------");
    // Bắt buộc phải khai báo trước kiểu dữ liệu nhỉ?
    let mut methods: Vec<Box<dyn PaymentMethod>> =
        vec![Box::new(credit), Box::new(momo), Box::new(crypto)]; // không còn dùng lẻ 3 method nữa nên move luôn vào

    pay_all(&mut methods, 10_000_000);
}

trait PaymentMethod {
    fn get_method_name(&self) -> &str;

    fn process_payment(&mut self, amount: u64) -> Result<String, String>;
}

#[derive(Debug)]
struct CreditCard {
    card_number: String,
    holder_name: String,
}

#[derive(Debug)]
struct MomoWallet {
    phone: String,
    balance: u64,
}

#[derive(Debug)]
struct CryptoWallet {
    wallet_address: String,
}

impl PaymentMethod for CreditCard {
    fn get_method_name(&self) -> &str {
        "Credit Card"
    }

    fn process_payment(&mut self, amount: u64) -> Result<String, String> {
        if amount > 50_000_000 {
            Err("Vượt hạn mức thẻ tín dụng".to_string())
        } else {
            Ok(format!(
                "Thanh toán thành công bằng {}, số thẻ {}, số tiền {amount}đ. Cảm ơn {} đã sử dụng dịch vụ",
                self.get_method_name(),
                self.card_number,
                self.holder_name
            ))
        }
    }
}

impl PaymentMethod for MomoWallet {
    fn get_method_name(&self) -> &str {
        "Momo Wallet"
    }

    fn process_payment(&mut self, amount: u64) -> Result<String, String> {
        if self.balance < amount {
            Err("Ví Momo không đủ số sư".to_string())
        } else {
            self.balance -= amount; // đã kiểm tra amount <= balance nên trừ thẳng thoải mái

            Ok(format!(
                "Thanh toán thành công bằng {}, số ví {}, số tiền {amount}đ",
                self.get_method_name(),
                self.phone
            ))
        }
    }
}

impl PaymentMethod for CryptoWallet {
    fn get_method_name(&self) -> &str {
        "Crypto Wallet"
    }

    fn process_payment(&mut self, amount: u64) -> Result<String, String> {
        Ok(format!(
            "Thanh toán thành công bằng {}, địa chỉ ví {}, số tiền {amount}đ",
            self.get_method_name(),
            self.wallet_address
        ))
    }
}

fn pay_single<T: PaymentMethod>(method: &mut T, amount: u64) -> Result<String, String> {
    method.process_payment(amount)
}

fn run_pay_single(method: &mut impl PaymentMethod, amount: u64) {
    // Đổi cách viết 1 tí
    match pay_single(method, amount) {
        Ok(message) => println!("{message}"),
        Err(mesage) => println!("{mesage}"),
    }
}

fn pay_all(methods: &mut [Box<dyn PaymentMethod>], amount: u64) {
    // Ban đầu tôi địng tận dụng run_pay_single() vào vòng for nhưng method không cùng kiểu dữ liệu
    for method in methods {
        match method.process_payment(amount) {
            Ok(message) => println!("{message}"),
            Err(mesage) => println!("{mesage}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_card_payment() {
        let mut credit = CreditCard {
            card_number: "4508 7500 1574 1019".to_string(),
            holder_name: "Tao".to_string(),
        };

        assert!(matches!(pay_single(&mut credit, 50_000_000), Ok(_)));
        assert!(matches!(pay_single(&mut credit, 50_000_001), Err(_)));
    }

    #[test]
    fn test_momo_payment() {
        let mut momo = MomoWallet {
            phone: "0123456789".to_string(),
            balance: 5_000_000,
        };

        assert_eq!(momo.balance, 5_000_000);

        assert!(matches!(pay_single(&mut momo, 4_000_000), Ok(_)));
        assert_eq!(momo.balance, 1_000_000);

        assert!(matches!(pay_single(&mut momo, 2_000_000), Err(_)));
        assert_eq!(momo.balance, 1_000_000);
    }

    #[test]
    fn test_dynamic_vec() {
        let credit = CreditCard {
            card_number: "4508 7500 1574 1019".to_string(),
            holder_name: "Tao".to_string(),
        };
        let momo = MomoWallet {
            phone: "0123456789".to_string(),
            balance: 5_000_000,
        };
        let crypto = CryptoWallet {
            wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        };

        let methods: Vec<Box<dyn PaymentMethod>> =
            vec![Box::new(credit), Box::new(momo), Box::new(crypto)];

        let names = vec!["Credit Card", "Momo Wallet", "Crypto Wallet"];

        for (i, method) in methods.iter().enumerate() {
            assert_eq!(Some(method.get_method_name()), names.get(i).copied())
        }
    }
}
