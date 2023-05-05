pub fn delete_and_backspace(s: &mut String) {
    let mut stack = String::new();
    let mut del = 0;
    for c in s.chars() {
        if del > 0 && c != '+' {
            del -= 1;
        } else if c == '-' {
            stack.pop();
        } else if c == '+' {
            del += 1;
        } else {
            stack.push(c)
        }
    }
    *s = stack;
}

pub fn do_operations(v: &mut Vec<String>) {

    for s in v.iter_mut() {
        if s.contains("+") {
            let add: Vec<&str> = s.split("+").collect();
            let first = add[0].parse::<i32>().unwrap();
            let second = add[1].parse::<i32>().unwrap();
            *s = (first + second).to_string();
        }
        if s.contains("-") {
            let add: Vec<&str> = s.split("-").collect();
            let first = add[0].parse::<i32>().unwrap();
            let second = add[1].parse::<i32>().unwrap();
            *s = (first - second).to_string();
        }
    }
}
