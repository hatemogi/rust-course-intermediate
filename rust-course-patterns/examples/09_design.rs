// 개선 전후의 분기 구조를 비교하기 위해 중첩된 if let을 유지합니다.
#[allow(clippy::collapsible_if, clippy::collapsible_match)]
fn retry_nested_if(input: Option<Result<u8, &str>>) -> bool {
    if let Some(result) = input {
        if let Ok(attempt) = result {
            if (1..=3).contains(&attempt) {
                return true;
            }
        }
    }
    false
}

fn retry_one_match(input: Option<Result<u8, &str>>) -> bool {
    matches!(input, Some(Ok(1..=3)))
}

#[derive(Clone, Copy)]
enum State {
    Waiting,
    Paused,
    Running,
}

struct Job {
    state: State,
    failures: u8,
}

fn should_retry_before(job: &Job) -> bool {
    matches!(
        job,
        Job {
            state: State::Waiting | State::Paused,
            failures: 1..=3,
        }
    )
}

fn can_start(state: &State) -> bool {
    matches!(state, State::Waiting | State::Paused)
}

fn should_retry_after(job: &Job) -> bool {
    can_start(&job.state) && (1..=3).contains(&job.failures)
}

fn use_rules() -> Job {
    let mut job = Job {
        state: State::Paused,
        failures: 2,
    };

    if should_retry_after(&job) {
        job.state = State::Running;
    }

    job
}

fn main() {
    let job = use_rules();

    assert!(matches!(job.state, State::Running));
    assert!(!can_start(&job.state));
    assert!(!should_retry_after(&job));

    for (input, expected) in [
        (None, false),
        (Some(Err("입력 없음")), false),
        (Some(Ok(0)), false),
        (Some(Ok(1)), true),
        (Some(Ok(3)), true),
        (Some(Ok(4)), false),
    ] {
        assert_eq!(retry_nested_if(input), expected);
        assert_eq!(retry_one_match(input), expected);
    }
    for state in [State::Waiting, State::Paused, State::Running] {
        for failures in [0, 1, 3, 4] {
            let expected = !matches!(state, State::Running) && (1..=3).contains(&failures);
            let job = Job { state, failures };
            assert_eq!(should_retry_before(&job), expected);
            assert_eq!(should_retry_after(&job), expected);
        }
    }
}
