#[allow(clippy::vec_init_then_push)] // Vec::new와 push를 단계별로 소개합니다.
fn values_in_steps() -> Vec<i32> {
    let mut values = Vec::new();
    values.push(10);
    values.push(20);
    values
}

fn main() {
    assert_eq!(values_in_steps(), vec![10, 20]);
}
