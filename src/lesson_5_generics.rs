use std::fmt::Display;

pub fn run() {
    let p1 = Product {
        name: "Mô hình siêu nhân".to_string(),
        price: 500_000,
    };
    let p2 = Product {
        name: "Mô hình công chúa".to_string(),
        price: u32::MAX,
    };
    let p3 = Product {
        name: "Mô hình thằng bạn".to_string(),
        price: 0,
    };

    let e1 = Employee {
        name: "Lượng Đức Trọng".to_string(),
        department: "Eat and Sleep department".to_string(),
    };

    println!("------------------");
    println!("{p1}");
    println!("{e1}");

    println!("------------------");
    let ps = vec![p1, p2, p3]; // p1, p2, p3 moved ở đây

    let best_models = filter_by(&ps, |model| model.price > 100_000);

    for model in &best_models {
        println!("{model}");
    }

    println!("------------------");
    print_all(&ps);

    println!("------------------");
    let page = Page {
        items: ps, // ps moved ở đây
        page_index: 0,
    };

    page.display_page();
}

trait Describable {
    fn sumarize(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
struct Product {
    name: String,
    price: u32,
}

#[derive(Debug)]
struct Employee {
    name: String,
    department: String,
}

struct Page<T> {
    items: Vec<T>,
    page_index: usize,
}

impl Describable for Product {
    fn sumarize(&self) -> String {
        format!("Sản phẩm: {} ({}đ)", self.name, self.price)
    }
}

impl Describable for Employee {
    fn sumarize(&self) -> String {
        format!("Nhân viên: {} ({})", self.name, self.department)
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sumarize())
    }
}

impl Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sumarize())
    }
}

impl<T: Display> Page<T> {
    fn format_page(&self) -> String {
        let header_str = format!("--- Trang {} ---\n", self.page_index + 1);

        let items_str = self
            .items
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        format!("{header_str}{items_str}")
    }

    // Bạn đã chỉ tôi tách dữ liệu ra khỏi side-effect để tiện test
    fn display_page(&self) {
        println!("{}", self.format_page());
    }
}

fn print_all<T: Display>(inputs: &[T]) {
    inputs
        .iter()
        // Tôi tự tìm hiểu cái này
        .enumerate()
        .for_each(|(i, x)| println!("{}. {x}", i + 1));
}

fn filter_by<T: Clone, F>(inputs: &[T], filter_fn: F) -> Vec<T>
where
    F: Fn(&T) -> bool, // đoạn này tôi cũng phải nhờ cứu viện chứ không biết cú pháp Fn(&T) -> bool
{
    inputs
        .iter()
        .filter(|i| filter_fn(i))
        // Dùng cloned để đổi &T -> T
        .cloned()
        .collect::<Vec<_>>()
    // đánh giá: hơi lú
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_by() {
        let p1 = Product {
            name: "Mô hình siêu nhân".to_string(),
            price: 500_000,
        };
        let p2 = Product {
            name: "Mô hình công chúa".to_string(),
            price: u32::MAX,
        };
        let p3 = Product {
            name: "Mô hình thằng bạn".to_string(),
            price: 0,
        };

        let ps = vec![p1.clone(), p2.clone(), p3.clone()];
        let results = filter_by(&ps, |model| model.price > 100_000);

        assert_eq!(results.len(), 2);
        // Bạn (session khác) vừa chỉ tôi so sánh Vec
        assert_eq!(results, vec![p1.clone(), p2.clone()]);
    }

    #[test]
    fn test_page() {
        let p1 = Product {
            name: "Mô hình siêu nhân".to_string(),
            price: 500_000,
        };
        let p2 = Product {
            name: "Mô hình công chúa".to_string(),
            price: u32::MAX,
        };

        let ps_page = Page {
            page_index: 0,
            items: vec![p1, p2],
        };

        let e1 = Employee {
            name: "Lượng Đức Trọng".to_string(),
            department: "Phòng ban phụ trách ăn và ngủ".to_string(),
        };
        let e2 = Employee {
            name: "Trọng Lượng Đức".to_string(),
            department: "Phòng ban phụ trách ngủ và ăn".to_string(),
        };

        let es_page = Page {
            page_index: 1,
            items: vec![e1, e2],
        };

        assert!(ps_page.format_page().contains("--- Trang 1 ---"));
        assert!(
            es_page
                .format_page()
                .contains("Nhân viên: Lượng Đức Trọng (Phòng ban phụ trách ăn và ngủ)")
        );
    }
}
