use std::fmt::Display;

pub fn run() {
    let command_1 = match parse_command("INVOKE window_open payload:width=800") {
        Ok(command) => command,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    let command_2 = match parse_command("QUERY get_profile") {
        Ok(command) => command,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    println!("{command_1:?}\n{command_2:?}");

    print_command(&command_1);
    print_command(&command_2);

    let Some(payload) = pick_longer_payload(&command_1, &command_2) else {
        println!("Không có payload hợp lệ");
        return;
    };

    println!("Longer payload: {}", payload);

    // THỬ NGHIỆM
    // let com_temp; // 1. Khai báo biến ở scope ngoài
    // {
    // let temp = String::from("QUERY temp_cmd"); // 2. `temp` được cấp phát trên Heap
    // com_temp = parse_command(&temp).unwrap(); // 3. `com_temp` mượn lát cắt &str từ `temp`
    // } // 4. Hết scope: `temp` bị DROP và thu hồi bộ nhớ ngay lập tức!

    // print_command(&com_temp); // BORROW CHECKER VẢ NGAY LẬP TỨC:
    // `temp` does not live long enough
    // `temp` dropped here while still borrowed

    // 6.2
    emit_event(100_i32);
    emit_event(String::from("User logged in"));
    emit_event("SYSTEM_READY");

    // Angering the compiler
    let local = String::from("local");
    // let slice: &str = &local;

    // local` does not live long enough
    // borrowed value does not live long enoughrustcClick for full compiler diagnostic

    // emit_event(slice);

    let boxed = leak_runtime_string(local);
    emit_event(boxed);
}

#[derive(Debug)]
struct CommandView<'a> {
    action: &'a str,
    target: &'a str,
    payload: Option<&'a str>,
}

impl Display for CommandView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action: {}; Target: {}", self.action, self.target)?;
        if let Some(payload) = self.payload {
            write!(f, "; Payload: {}", payload)?;
        }
        Ok(())
    }
}

fn parse_command(raw: &str) -> Result<CommandView<'_>, &str> {
    let mut strings = raw.split_whitespace();

    let action = strings
        .next()
        .ok_or("Lệnh không đúng định dạng tối thiểu (thiếu action hoặc target)")?;
    let target = strings
        .next()
        .ok_or("Lệnh không đúng định dạng tối thiểu (thiếu action hoặc target)")?;

    let payload = strings
        .next()
        // and_then() phù hợp khi closure trả về Option
        // map() phù hợp khi closure trả về giá trị thông thường
        .and_then(|p| p.strip_prefix("payload:").filter(|&p| !p.trim().is_empty()));

    Ok(CommandView {
        action,
        target,
        payload,
    })
}

fn pick_longer_payload<'a>(cm1: &CommandView<'a>, cm2: &CommandView<'a>) -> Option<&'a str> {
    match (cm1.payload, cm2.payload) {
        (Some(p1), Some(p2)) => {
            if p1.len() >= p2.len() {
                Some(p1)
            } else {
                Some(p2)
            }
        }
        (Some(p1), None) => Some(p1),
        (None, Some(p2)) => Some(p2),
        (None, None) => None,
    }
}

fn print_command(command: &CommandView<'_>) {
    println!("{command}");
}

fn emit_event<T>(event: T)
where
    T: std::fmt::Debug + Display + 'static,
{
    let type_name = std::any::type_name::<T>();
    println!("Type name: {type_name} - Value: {event}");
}

fn leak_runtime_string(string: String) -> &'static str {
    Box::leak(string.into_boxed_str())
}
