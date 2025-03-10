use std::time::Duration;
use trpl;

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

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    })
}
