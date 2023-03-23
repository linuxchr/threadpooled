use std::thread::sleep;
use std::time::Duration;
use threadpooled;

#[test]

fn add() {
    let mut pool = threadpooled::Threadpool::new(4, 1);
    let mut t = pool
        .assing(|| {
            let _ = 1 + 1;
        })
        .unwrap();
    sleep(Duration::from_secs(1));
    if !t.is_finished().unwrap() {
        panic!("untrue");
    }
    assert!(true);
}
