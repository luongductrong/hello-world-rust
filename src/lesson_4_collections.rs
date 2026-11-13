use std::collections::HashMap;

pub fn run() {
    expense_management();
    text_analyzer();
}

// Ban đầu, khi viết Vec, tôi tạo Enum dùng cho tiện
// Đến khi đọc đề HashMap, thấy phải dùng Category làm key, tôi: "Ủa, enum có làm key được không ta?"
// Tra cứu thử thì thấy là được, nhưng phải impl các Trait này
// Và compiler cũng yêu cầu impl Clone và Copy để hàm insert/entry trong HashMap không move mất enum
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Category {
    FoodAndDrink,
    Transportation,
    Education,
}

#[derive(Debug)]
struct Expense {
    title: String,
    category: Category,
    amount: u32,
}

impl Category {
    fn describe(&self) -> &str {
        match self {
            Self::FoodAndDrink => "ăn bậy ăn bạ",
            Self::Education => "học này học nọ",
            Self::Transportation => "đi đây đi đó",
        }
    }
}

impl Expense {
    fn describe(&self) -> String {
        format!(
            "À thực ra khoản {} này anh chi cho {}, cỡ {}đ",
            self.title,
            self.category.describe(),
            self.amount
        )
    }
}

fn expense_management() {
    let e1: Expense = Expense {
        title: String::from("Tiền ăn gia đình"),
        category: Category::FoodAndDrink,
        amount: 500_000,
    };
    let e2: Expense = Expense {
        title: String::from("Tiền học gãi lưng"),
        category: Category::Education,
        amount: 2_000_000,
    };
    let e3: Expense = Expense {
        title: String::from("Tiền đổ xăng xe đạp"),
        category: Category::Transportation,
        amount: 10_000_000,
    };
    let e4: Expense = Expense {
        title: String::from("Tiền pate cho mèo hàng xóm"),
        category: Category::FoodAndDrink,
        amount: 20_000_000,
    };
    let e5: Expense = Expense {
        title: String::from("Tiền hội người trẻ neo đơn nhiều nơi nương tựa"),
        category: Category::Education,
        amount: 2_000,
    };

    let mut expenses: Vec<Expense> = vec![e1, e2, e3, e4];

    // Vợ hỏi mèo hàng xóm là cái hay đực, xem thử chi bao nhiêu tiền
    match expenses.get(3) {
        Some(expense) => println!("{}", expense.describe()),
        None => println!("Chịu, làm gì có, em phải tin anh"),
    }

    // Chắc ảnh chi khoản nào nữa chứ chẳng lẽ chi có 4 khoản
    match expenses.get(99) {
        Some(expense) => println!("{}", expense.describe()),
        None => println!("Chịu, làm gì có, em phải tin anh"),
    }

    fn safe_remove(expenses: &mut Vec<Expense>, index: usize) -> Option<Expense> {
        // Ban đầu tôi viết là if let Some(_) = expenses.get(index) nhưng clippy bảo sửa thành:
        if expenses.get(index).is_some() {
            return Some(expenses.remove(index));
        }
        None
    }

    // Thôi xoá bớt kẻo vợ lại hỏi
    match safe_remove(&mut expenses, 1) {
        Some(ex) => println!(
            "Tuyệt, xoá được rồi, cái khoản {} gì mà tận {}đ, xoá kẻo vợ nghi",
            ex.title, ex.amount
        ),
        None => println!("Ủa?"),
    }

    // Hình như mình chi hơi nhiều, tém tém lại
    for ex in &mut expenses {
        ex.amount = ex.amount.saturating_sub(500_000);
    }

    // Để xem lại thử
    for ex in &expenses {
        println!("{}", ex.describe());
    }

    // Khoản đầu tiên là gì ấy ta
    let ex1 = &expenses[0];
    println!("{}", ex1.describe());

    expenses.push(e5);

    // Nếu đặt push lên khi ex1 còn mượn => Lỗi
    // cannot borrow `expenses` as mutable because it is also borrowed as immutable
    // lesson_4_collections.rs(107, 16): immutable borrow occurs here
    // lesson_4_collections.rs(111, 20): immutable borrow later used here
    // mutable borrow occurs hererustcClick for full compiler diagnostic
    // Fix => Đưa lệnh print dùng ex1 lên trên push

    // HASH MAP
    // fn total_amount_by_category(expenses: &Vec<Expense>) -> HashMap<Category, u32> {
    fn total_amount_by_category(expenses: &[Expense]) -> HashMap<Category, u32> {
        let mut totals: HashMap<Category, u32> = HashMap::new();

        for ex in expenses {
            // Và tôi cũng học thêm được cách dùng .and_modify() thay vì dùng let để lấy ra như bạn chỉ
            // totals.entry(ex.category).and_modify(|total| *total += ex.amount).or_insert(0);
            // Nhưng cách trên sẽ tính thiếu lần đầu (1 sesson khác của bạn đã nhắc nhở)
            totals
                .entry(ex.category)
                .and_modify(|total| *total += ex.amount)
                .or_insert(ex.amount);
        }
        totals
    }

    // lấy totals ra dùng chung
    let totals = total_amount_by_category(&expenses);

    // Ồ, thì ra 1 phần tử là 1 Tuple
    for (cate, total) in &totals {
        println!("Khoảng {}, tổng cộng: {}đ", cate.describe(), total);
    }

    //
    println!("\nDòng này để đánh dấu, lúc mở terminal lên đỡ lú :)))");
    let f_and_d = totals.get(&Category::FoodAndDrink).copied().unwrap_or(0);
    println!(
        "Đã chi cho {} {}đ",
        Category::FoodAndDrink.describe(),
        f_and_d
    );
}

fn text_analyzer() {
    let text = "rust rất hay nhưng rust cũng rất khó";

    let mut counter: HashMap<&str, u32> = HashMap::new();

    for t in text.split_whitespace() {
        counter
            .entry(t)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    println!("\n{:<10} Số lần xuất hiện", "Từ");
    for (text, count) in counter {
        println!("{:<10} {}", text, count);
    }
}
