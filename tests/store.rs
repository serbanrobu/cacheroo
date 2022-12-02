use cacheroo::Store;
use tokio::{
    task,
    time::{self, Duration},
};

#[tokio::test]
async fn test_insert_and_get() {
    let store = Store::new();
    let k = "test";
    let v = "insert_and_get";

    store.insert(k, v).await;

    assert_eq!(store.get(k).await, Some(v));
}

#[tokio::test(start_paused = true)]
async fn test_lifetime() {
    let store = Store::new();
    let duration = Duration::from_secs(1);
    let k = "test";
    let v = "lifetime";

    store.insert_with_lifetime(k, v, duration).await;

    assert_eq!(store.get(k).await, Some(v));

    time::sleep(duration).await;
    task::yield_now().await;

    assert_eq!(store.get(k).await, None);
}

#[tokio::test]
async fn test_remove() {
    let store = Store::new();
    let k = "test";
    let v = "remove";

    store.insert(k, v).await;

    assert!(store.contains_key(k).await);

    store.remove(k).await;

    assert!(!store.contains_key(k).await);
}

#[tokio::test(start_paused = true)]
async fn test_abort_expiration() {
    let store = Store::new();
    let duration_1 = Duration::from_secs(1);
    let duration_2 = duration_1 + Duration::from_secs(1);
    let k = "test";
    let v = "abort_expiration";

    store.insert_with_lifetime(k, v, duration_1).await;
    store.insert_with_lifetime(k, v, duration_2).await;

    assert_eq!(store.get(k).await, Some(v));

    time::sleep(duration_1).await;

    assert_eq!(store.get(k).await, Some(v));

    time::sleep(duration_2).await;

    assert_eq!(store.get(k).await, None);
}

#[tokio::test(start_paused = true)]
async fn test_zero_lifetime() {
    let store = Store::new();
    let k = "test";
    let v = "zero_lifetime";

    store.insert_with_lifetime(k, v, Duration::ZERO).await;

    assert_eq!(store.get(k).await, Some(v));

    task::yield_now().await;

    assert_eq!(store.get(k).await, None);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_multithreading() {
    let store = Store::new();
    let mut handles = vec![];

    let k_1 = "one";
    let v_1 = 1;
    let store_1 = store.clone();

    let handle_1 = task::spawn(async move {
        store_1.insert(k_1, v_1).await;
    });

    handles.push(handle_1);

    let k_2 = "two";
    let v_2 = 2;
    let store_2 = store.clone();

    let handle_2 = task::spawn(async move {
        store_2.insert(k_2, v_2).await;
    });

    handles.push(handle_2);

    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(store.get(k_1).await, Some(v_1));
    assert_eq!(store.get(k_2).await, Some(v_2));
}

#[ignore]
#[tokio::test]
async fn test_insert_ten_million() {
    let store = Store::new();

    for i in 0..10_000_000 {
        store.insert(i, i).await;
    }
}
