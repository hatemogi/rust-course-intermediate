fn main() {
    let message = String::from("완료");
    print_message(message);
    println!("다시 출력: {message}");
}

fn print_message(message: String) {
    println!("{message}");
}
