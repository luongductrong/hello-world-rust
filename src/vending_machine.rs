use std::io::{self, Write};

const PWORD: &str = "aZaZ";

pub fn run() {
    let slot1 = DrinkSlot::new(1, "Nước suối", 10_000, 2);
    let slot2 = DrinkSlot::new(2, "Trà đào", 25_000, 1);
    let slot3 = DrinkSlot::new(3, "Cà phê", 30_000, 0);

    let mut machine = VendingMachine::new([slot1, slot2, slot3]);

    let mut step = StepMenu::Main;

    loop {
        match step {
            StepMenu::Main => {
                println!(
                    "\nChào mừng quý khách:\n  1. Nạp tiền\n  2. Hoàn tiền thừa\n  3. Xem số dư\n  4. Xem menu & Mua hàng\n  <PassWord>. Kết sổ, tắt máy"
                );

                let Ok(input) = read_line_trimmed("Nhập lựa chọn: ") else {
                    println!("Opps. Tôi đình công rồi, thử lại sau nhé\n");
                    break;
                };

                match input.as_str() {
                    PWORD => {
                        machine.close();
                        break;
                    }
                    "1" => step = StepMenu::TopUp,
                    "2" => match machine.withdraw() {
                        Some(amount) => println!("Đã rút thành công {amount}đ"),
                        None => println!("Thất bại, làm gì có tiền mà rút"),
                    },
                    "3" => machine.view_balance(),
                    "4" => step = StepMenu::Drink,
                    _ => println!("Biết đọc không cưng?\n"),
                }
                continue;
            }
            StepMenu::Drink => {
                machine.view_menu();
                println!("    q. Quay lại");

                let Ok(input) = read_line_trimmed("Nhập lựa chọn: ") else {
                    println!("Opps. Tôi đình công rồi, thử lại sau nhé\n");
                    break;
                };

                if input == "q" {
                    step = StepMenu::Main;
                    continue;
                }

                let drink_id: u8 = match input.parse() {
                    Ok(out) => out,
                    Err(_) => {
                        println!("Lựa chọn không hợp lệ");
                        continue;
                    }
                };

                match machine.buy_drink(drink_id) {
                    Ok((drink_name, remaining_balance)) => {
                        println!(
                            "Mua thành công {drink_name}\nSố dư còn lại là {remaining_balance}đ"
                        );
                        step = StepMenu::Main;
                    }
                    Err(err) => match err {
                        MachineError::NotFound { id } => {
                            println!("Chỗ chụy không có món số {id} này cưng ơi");
                        }
                        MachineError::OutOfStock { name } => {
                            println!("Ui, mình hết {name} rồi bạn ơi, thử lại món khác nhé");
                        }
                        MachineError::NotEnoughMoney {
                            drink_price,
                            remaining_balance,
                        } => {
                            println!(
                                "Nạp thêm đi trời, món này {drink_price}đ, nạp thêm ít nhất {}đ nữa nhé",
                                drink_price - remaining_balance
                            );
                        }
                        MachineError::Format(str) => {
                            println!("Chịu: {str}")
                        }
                    },
                }
                continue;
            }
            StepMenu::TopUp => {
                println!(
                    "\nMến chào phú ông phú bà, nạp nhiều nhiều vào nhé!\nNhập số tiền muốn nộp hoặc \"q\" để quay lại."
                );

                let Ok(input) = read_line_trimmed("Nhập số tiền: ") else {
                    println!("Opps. Tôi đình công rồi, thử lại sau nhé\n");
                    break;
                };

                if input == "q" {
                    step = StepMenu::Main;
                    continue;
                }

                let input: u32 = match input.parse() {
                    Ok(out) => out,
                    Err(_) => {
                        println!("Lựa chọn không hợp lệ");
                        continue;
                    }
                };

                if input == 0 {
                    println!("Nạp 0đ là đã muốn nạp chưa? Hay không có tiền?");
                    continue;
                }

                let Some((amount, new_balance)) = machine.top_up(input) else {
                    println!(
                        "Opps, chúng tôi bán nước chứ không rửa tiền. Bạn có cần địa chỉ ngân hàng gần nhất không?"
                    );
                    continue;
                };

                println!("Đã nạp thành công {amount}đ, số dư hiện tại là {new_balance}đ");

                step = StepMenu::Main;
                continue;
            }
        }
    }
}

#[derive(Debug)]
enum StepMenu {
    Main,
    Drink,
    TopUp,
}

#[derive(Debug)]
struct DrinkSlot {
    id: u8,
    name: String,
    price: u32,
    quantity: u32,
}

#[derive(Debug)]
struct VendingMachine {
    slots: [DrinkSlot; 3],
    balance: u32,
    revenue: u32,
}

#[derive(Debug)]
enum MachineError {
    Format(String),
    NotFound {
        id: u8,
    },
    OutOfStock {
        name: String,
    },
    NotEnoughMoney {
        drink_price: u32,
        remaining_balance: u32,
    },
}

impl DrinkSlot {
    fn new(id: u8, name: &str, price: u32, quantity: u32) -> Self {
        Self {
            id,
            name: String::from(name),
            price,
            quantity,
        }
    }

    fn describe(&self) -> String {
        let quantity = self.quantity;
        let status = if quantity > 0 {
            format!("SL: {quantity}")
        } else {
            String::from("Hết hàng")
        };
        format!("#{} {:<9} - {}đ ({status})", self.id, self.name, self.price)
    }

    fn is_in_stock(&self) -> bool {
        self.quantity > 0
    }

    fn sell(&mut self) -> Result<(), MachineError> {
        if self.quantity >= 1 {
            self.quantity -= 1;
            return Ok(());
        }
        let drink_name = self.name.clone();
        Err(MachineError::OutOfStock { name: drink_name })
    }
}

impl VendingMachine {
    fn new(slots: [DrinkSlot; 3]) -> Self {
        Self {
            slots,
            balance: 0,
            revenue: 0,
        }
    }

    fn view_menu(&self) {
        println!("\n\nĐể xem hôm nay còn món gì, chắc tại ế:");
        for slot in &self.slots {
            println!("    {}", slot.describe());
        }
    }

    fn top_up(&mut self, amount: u32) -> Option<(u32, u32)> {
        match self.balance.checked_add(amount) {
            Some(new_balance) => {
                self.balance = new_balance;
                Some((amount, new_balance))
            }
            None => None,
        }
    }

    fn withdraw(&mut self) -> Option<u32> {
        let balance = self.balance;
        if balance > 0 {
            self.balance = 0;
            Some(balance)
        } else {
            None
        }
    }

    fn view_balance(&self) {
        println!("Số dư hiện tại là {}đ", self.balance);
    }

    fn close(self) {
        println!("-> Doanh thu tổng cộng: {}", self.revenue);
        println!("Chụy đi ngủ đây, không bán nữa, byeeeeee");
    }

    fn buy_drink(&mut self, id: u8) -> Result<(&str, u32), MachineError> {
        let Some(drink) = self.slots.iter_mut().find(|x| x.id == id) else {
            return Err(MachineError::NotFound { id });
        };
        if !drink.is_in_stock() {
            let drink_name = drink.name.clone();
            return Err(MachineError::OutOfStock { name: drink_name });
        }
        if drink.price > self.balance {
            return Err(MachineError::NotEnoughMoney {
                drink_price: drink.price,
                remaining_balance: self.balance,
            });
        }

        drink.sell()?;
        self.balance = self.balance.saturating_sub(drink.price);
        self.revenue = self.revenue.saturating_add(drink.price);

        Ok((&drink.name, self.balance))
    }
}

fn read_line_trimmed(prompt: &str) -> Result<String, MachineError> {
    print!("{prompt}");
    let mut input: String = String::new();

    let Ok(_) = io::stdout().flush() else {
        return Err(MachineError::Format(String::from("Đã xảy ra lỗi")));
    };

    match io::stdin().read_line(&mut input) {
        Ok(0) => return Err(MachineError::Format(String::from("Lỗi nhập liệu"))),
        Ok(_) => {}
        Err(_) => return Err(MachineError::Format(String::from("Không đọc được dữ liệu"))),
    }

    Ok(input.trim().to_string())
}
