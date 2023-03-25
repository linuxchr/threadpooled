use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;
use threadpooled;

#[test]

fn add() {
    let mut pool = threadpooled::Threadpool::new(4);
    let t = pool
        .assing(|| {
            let _ = 1 + 1;
        })
        .unwrap();
    sleep(Duration::from_secs(1));
    assert!(pool.is_finished(t).unwrap());
}

#[test]
fn multicore() {
    let mut pool = threadpooled::Threadpool::new(4);
    let t = pool
        .assing(|| {
            let _ = 1 + 1;
        })
        .unwrap();
    sleep(Duration::from_secs(1));
    assert!(pool.is_finished(t).unwrap());
    pool.assing(|| {
        let _ = 1 + 1;
    })
    .unwrap();
    pool.assing(|| {
        let _ = 1 + 1;
    })
    .unwrap();
    pool.assing(|| {
        let _ = 1 + 1;
    })
    .unwrap();
    pool.assing(|| {
        let _ = 1 + 1;
    })
    .unwrap();
}

#[test]
#[should_panic]
fn no_overthread() {
    let mut pool = threadpooled::Threadpool::new(4);
    pool.assing(|| {
        sleep(Duration::from_secs(60));
    })
    .unwrap();
    pool.assing(|| {
        sleep(Duration::from_secs(60));
    })
    .unwrap();
    pool.assing(|| {
        sleep(Duration::from_secs(60));
    })
    .unwrap();
    pool.assing(|| {
        sleep(Duration::from_secs(60));
    })
    .unwrap();
    pool.assing(|| {
        sleep(Duration::from_secs(60));
    })
    .unwrap();
}

#[test]
fn not_done() {
    let mut pool = threadpooled::Threadpool::new(4);
    let t = pool
        .assing(|| {
            sleep(Duration::from_secs(60));
        })
        .unwrap();
    assert!(!pool.is_finished(t).unwrap());
}

#[test]
fn join_test() {
    let mut pool = threadpooled::Threadpool::new(1);
    let (rx, tx) = mpsc::channel();
    let t = pool
        .assing(move || {
            sleep(Duration::from_secs(1));
            rx.send(true).unwrap();
        })
        .unwrap();
    pool.join(t).unwrap();
    assert!(tx.recv().unwrap());
    if let Ok(_) = pool.is_finished(t) {
        panic!();
    }
}
