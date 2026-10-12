fn borrow() {
    let name = Some(String::from("러스트"));
    let length = name.as_ref().map(|value| value.len());
    assert_eq!(length, Some(9));
    assert_eq!(name, Some(String::from("러스트")));
}

fn take() {
    let mut token = Some(String::from("secret"));
    let owned = token.take();
    assert_eq!(owned.as_deref(), Some("secret"));
    assert_eq!(token, None);
}

fn mutate() {
    let mut name = Some(String::from("러스트"));
    if let Some(value) = name.as_mut() {
        value.push('!');
    }
    assert_eq!(name, Some(String::from("러스트!")));
}

struct Worker {
    pending: Option<String>,
}

impl Worker {
    fn run(&mut self) {
        if let Some(job) = self.pending.take() {
            println!("작업 처리: {job}");
        }
    }
}

fn run_pending_job() {
    let mut worker = Worker {
        pending: Some(String::from("메일 보내기")),
    };
    worker.run();
    assert_eq!(worker.pending, None);
    worker.run();
    assert_eq!(worker.pending, None);
}

fn main() {
    mutate();
    borrow();
    take();
    run_pending_job();
}
