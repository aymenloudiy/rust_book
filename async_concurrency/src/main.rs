use std::{
    future::Future,
    pin::Pin,
    thread,
    time::{Duration, Instant},
};
use trpl::{self, Either};

fn main() {
    // trpl::run(async {
    //     let handle = trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });
    //     for i in 1..5 {
    //         println!("hi number {i} from the second task");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }
    //     handle.await.unwrap();
    // });
    // trpl::run(async {
    //     let (tx, mut rx) = trpl::channel();
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     let received = rx.recv().await.unwrap();
    //     println!("got {received}")
    // })
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // trpl::join3(tx1_fut, tx_fut, rx_fut).await;
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        trpl::join_all(futures).await;
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
        {
            fn slow(name: &str, ms: u64) {
                thread::sleep(Duration::from_millis(ms));
                println!("'{name}' ran for {ms}ms");
            }
            let a = async {
                println!("'a' started.");
                slow("a", 30);
                slow("a", 10);
                slow("a", 20);
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started.");
                slow("b", 75);
                slow("b", 10);
                slow("b", 15);
                slow("b", 350);
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'b' finished.");
            };

            trpl::race(a, b).await;
            let one_ms = Duration::from_millis(1);

            let _a = async {
                println!("'a' started.");
                slow("a", 30);
                trpl::sleep(one_ms).await;
                slow("a", 10);
                trpl::sleep(one_ms).await;
                slow("a", 20);
                trpl::sleep(one_ms).await;
                println!("'a' finished.");
            };

            let _b = async {
                println!("'b' started.");
                slow("b", 75);
                trpl::sleep(one_ms).await;
                slow("b", 10);
                trpl::sleep(one_ms).await;
                slow("b", 15);
                trpl::sleep(one_ms).await;
                slow("b", 35);
                trpl::sleep(one_ms).await;
                println!("'b' finished.");
            };
        }
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );
    });
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
