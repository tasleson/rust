//! Test to verify the #[exclusive] attribute works correctly

#![feature(exclusive_tests)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[test]
#[exclusive]
fn exclusive_test_1() {
    // This test should run alone
    let before = COUNTER.load(Ordering::SeqCst);
    assert_eq!(before, 0, "No other exclusive test should be running");
    COUNTER.store(1, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(100));
    let after = COUNTER.load(Ordering::SeqCst);
    assert_eq!(after, 1, "No other test modified the counter");
    COUNTER.store(0, Ordering::SeqCst);
}

#[test]
#[exclusive]
fn exclusive_test_2() {
    // This test should also run alone
    let before = COUNTER.load(Ordering::SeqCst);
    assert_eq!(before, 0, "No other exclusive test should be running");
    COUNTER.store(2, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(100));
    let after = COUNTER.load(Ordering::SeqCst);
    assert_eq!(after, 2, "No other test modified the counter");
    COUNTER.store(0, Ordering::SeqCst);
}

#[test]
#[exclusive]
fn exclusive_test_3() {
    // This test should also run alone
    let before = COUNTER.load(Ordering::SeqCst);
    assert_eq!(before, 0, "No other exclusive test should be running");
    COUNTER.store(3, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(100));
    let after = COUNTER.load(Ordering::SeqCst);
    assert_eq!(after, 3, "No other test modified the counter");
    COUNTER.store(0, Ordering::SeqCst);
}

// Regular parallel tests that can run concurrently
#[test]
fn parallel_test_1() {
    thread::sleep(Duration::from_millis(50));
    assert_eq!(1 + 1, 2);
}

#[test]
fn parallel_test_2() {
    thread::sleep(Duration::from_millis(50));
    assert_eq!(2 + 2, 4);
}

#[test]
fn parallel_test_3() {
    thread::sleep(Duration::from_millis(50));
    assert_eq!(3 + 3, 6);
}

// Test that exclusive works with other attributes
#[test]
#[exclusive]
#[should_panic]
fn exclusive_with_should_panic() {
    panic!("This should panic but run exclusively");
}

#[test]
#[exclusive]
#[ignore = "testing ignore with exclusive"]
fn exclusive_with_ignore() {
    // This test should be ignored
    assert!(false, "This should not run");
}
