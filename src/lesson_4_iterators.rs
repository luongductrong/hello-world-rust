use std::collections::HashMap;

pub fn run() {
    let i1 = OrderItem {
        name: "Chuối".to_string(),
        price: 2_000,
        quantity: 10,
    };
    let i2 = OrderItem {
        name: "Thịt gà".to_string(),
        price: 120_000,
        quantity: 2,
    };
    let i3 = OrderItem {
        name: "Gạo".to_string(),
        price: 100_000,
        quantity: 1,
    };
    let i4 = OrderItem {
        name: "Rau củ".to_string(),
        price: 20_000,
        quantity: 2,
    };
    let i5 = OrderItem {
        quantity: 20,
        ..i1.clone()
    };
    let i6 = OrderItem {
        name: "Sữa tươi".to_string(),
        price: 35_000,
        quantity: 2,
    };
    let i7 = OrderItem {
        name: "Trứng gà".to_string(),
        price: 30_000,
        quantity: 3,
    };
    let i8 = OrderItem {
        name: "Cá hồi".to_string(),
        price: 250_000,
        quantity: 1,
    };

    let orders = vec![
        Order {
            id: 1,
            items: vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()],
            status: OrderStatus::Delivered,
        },
        Order {
            id: 2,
            items: vec![
                i5.clone(),
                OrderItem {
                    quantity: 3,
                    ..i2.clone()
                },
                i6.clone(),
            ],
            status: OrderStatus::Delivered,
        },
        Order {
            id: 3,
            items: vec![
                OrderItem {
                    quantity: 15,
                    ..i1.clone()
                },
                OrderItem {
                    quantity: 2,
                    ..i3.clone()
                },
                i7.clone(),
            ],
            status: OrderStatus::Delivered,
        },
        Order {
            id: 4,
            items: vec![
                OrderItem {
                    quantity: 1,
                    ..i2.clone()
                },
                OrderItem {
                    quantity: 4,
                    ..i4.clone()
                },
                OrderItem {
                    quantity: 3,
                    ..i6.clone()
                },
            ],
            status: OrderStatus::Pending,
        },
        Order {
            id: 5,
            items: vec![i8.clone()],
            status: OrderStatus::Cancelled,
        },
    ];

    if let Some(order) = Order::find_order(&orders, 1) {
        println!(
            "Tổng số tiền đơn hàng {} là {}đ",
            order.id,
            order.total_price()
        );
    } else {
        println!("Không tìm thấy đơn hàng này");
    }

    println!(
        "Tổng doanh thu hệ thống là {}đ",
        Order::total_revenue(&orders)
    );

    let premium_items = Order::premium_order_items(&orders, 100_000);

    println!("\nPremium Items:");
    premium_items.iter().for_each(|item| println!("{item}"));

    let analyzer = Order::sales_analyzer(&orders);

    println!("\n{:<9} => Số lượng", "Sản phẩm");
    analyzer
        .iter()
        .for_each(|(key, value)| println!("{:<9} => {} cái", key, value));

    let analyzer2 = Order::sales_analyzer2(&orders);

    println!("\n{:<9} => Số lượng", "Sản phẩm");
    analyzer2
        .iter()
        .for_each(|(key, value)| println!("{:<9} => {} cái", key, value));
}

// Chưa dùng tới PartialEq, Eq
#[derive(Debug, Clone, Copy)]
enum OrderStatus {
    Pending,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone)]
struct OrderItem {
    name: String,
    price: u32,
    quantity: u32,
}

#[derive(Debug, Clone)]
struct Order {
    id: u32,
    items: Vec<OrderItem>,
    status: OrderStatus,
}

impl Order {
    fn total_price(&self) -> u64 {
        self.items
            .iter()
            .map(|item| u64::from(item.quantity.saturating_mul(item.price)))
            .sum()
    }

    // u64 đã rất lớn rồi
    fn total_revenue(orders: &[Order]) -> u64 {
        orders
            // order ở đây là &Order
            .iter()
            // order ở filter này là &&Order
            .filter(|order| matches!(order.status, OrderStatus::Delivered))
            // vậy sao ở đây là &Order chứ không phải &&&Order?
            .map(|order| order.total_price())
            .sum()
    }

    fn premium_order_items(orders: &[Order], min_price: u32) -> Vec<&str> {
        let mut item_names = orders
            .iter()
            .filter(|order| matches!(order.status, OrderStatus::Delivered))
            .flat_map(|order| order.items.iter())
            .filter(|item| item.price >= min_price)
            // Dùng as_str()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>();

        item_names.sort();
        item_names.dedup();

        item_names
    }

    fn find_order(orders: &[Order], id: u32) -> Option<&Order> {
        orders.iter().find(|order| order.id == id)
    }

    fn sales_analyzer(orders: &[Order]) -> HashMap<&str, u32> {
        let mut analyzer: HashMap<&str, u32> = HashMap::new();

        for order in orders {
            if let OrderStatus::Delivered = order.status {
                for item in &order.items {
                    analyzer
                        .entry(&item.name)
                        .and_modify(|x| *x += item.quantity)
                        .or_insert(item.quantity);
                }
            }
        }
        analyzer
    }

    fn sales_analyzer2(orders: &[Order]) -> HashMap<&str, u32> {
        orders
            .iter()
            .filter(|order| matches!(order.status, OrderStatus::Delivered))
            .flat_map(|order| order.items.iter())
            .fold(HashMap::new(), |mut acc, item| {
                // .or_insert(0) trả về &mut u32 trỏ vào value trong map
                *acc.entry(&item.name).or_insert(0) += item.quantity;
                acc // trả lại map tích lũy cho lượt sau
            })
    }
}
