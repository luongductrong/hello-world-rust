pub fn run() {
    let mut account = Account::new("trongld", 100_000);

    fn test(account: &mut Account, str: &str) {
        match checkout_pipeline(account, str) {
            Ok(balance) => println!("Thành công, số dư còn lại {balance}đ"),
            Err(PaymentError::ZeroAmount) => {
                println!("Số tiền tối thiểu lớn hơn 0đ");
            }
            Err(PaymentError::InvalidAmountString(message)) => {
                println!("Lỗi: {message}");
            }
            Err(PaymentError::InsufficientFunds { balance, required }) => {
                println!("Không đủ {required}đ, số dư hiện tại là {balance}đ")
            }
        }
    }

    println!("Current username: {}", account.username);
    test(&mut account, "50000");
    test(&mut account, "hai_muoi_nghin");
    test(&mut account, "80000");
}

#[derive(Debug)]
struct Account {
    username: String,
    balance: u64,
}

#[derive(Debug)]
enum PaymentError {
    InvalidAmountString(String),
    ZeroAmount,
    InsufficientFunds { balance: u64, required: u64 },
}

impl Account {
    fn new(username: &str, balance: u64) -> Self {
        Self {
            username: username.to_string(),
            balance,
        }
    }
}

fn parse_amount(str: &str) -> Result<u64, PaymentError> {
    let amount = str
        .trim()
        .parse::<u64>()
        .map_err(|_| PaymentError::InvalidAmountString("Số tiền không hợp lệ".to_string()))?;

    if amount == 0 {
        Err(PaymentError::ZeroAmount)
    } else {
        Ok(amount)
    }
}

fn deduct_balance(account: &mut Account, amount: u64) -> Result<u64, PaymentError> {
    if account.balance < amount {
        Err(PaymentError::InsufficientFunds {
            balance: account.balance,
            required: amount,
        })
    } else {
        account.balance -= amount;
        Ok(account.balance)
    }
}

fn checkout_pipeline(account: &mut Account, raw_amount: &str) -> Result<u64, PaymentError> {
    // phải ? để lấy được giá trị u64
    let amount = parse_amount(raw_amount)?;

    // deduct_balance không cần phải ? vì nó chủ động return Result
    deduct_balance(account, amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_amount_ok() -> Result<(), PaymentError> {
        let result = parse_amount(" 50000 ")?;

        assert_eq!(result, 50_000);
        Ok(())
    }

    #[test]
    fn test_parse_amount_zero() {
        let result = parse_amount(" 0");
        assert!(matches!(result, Err(PaymentError::ZeroAmount)));
    }

    #[test]
    fn test_parse_amount_invalid_string() {
        let result = parse_amount("uia_uia_ia");
        assert!(matches!(result, Err(PaymentError::InvalidAmountString(_))));
    }

    #[test]
    fn test_deduct_balance_ok() -> Result<(), PaymentError> {
        let mut acc = Account::new("trongld", 100_000);

        let result = deduct_balance(&mut acc, 40_000)?;

        assert_eq!(result, 60_000);
        assert_eq!(acc.balance, 60_000);
        Ok(())
    }

    #[test]
    fn test_deduct_balance_err() {
        let mut acc = Account::new("trongld", 50_000);

        let result = deduct_balance(&mut acc, 80_000);

        assert!(matches!(
            result,
            Err(PaymentError::InsufficientFunds { .. })
        ));

        if let Err(PaymentError::InsufficientFunds { balance, .. }) = result {
            assert_eq!(balance, 50_000);
            assert_eq!(acc.balance, 50_000);
        }
    }

    #[test]
    fn test_checkout_pipeline() -> Result<(), PaymentError> {
        let mut acc = Account::new("trongld", 100_000);

        let result = checkout_pipeline(&mut acc, "  70000")?;

        assert_eq!(result, 30_000);
        Ok(())
    }
}
