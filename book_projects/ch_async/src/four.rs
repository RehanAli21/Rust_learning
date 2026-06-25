use std::thread;
use std::time::Duration;

use trpl::Either;

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn main() {
    //problem example
    //shows that b have to wait for a to finish
    trpl::block_on(async {
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

        trpl::select(a, b).await;
    });

    println!("______\n_____\n_____");

    //solution example
    trpl::block_on(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });

    println!("______\n_____\n_____");

    println!("______\n_____\n_____");

    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(1)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });

    println!("______\n_____\n_____");

    println!("______\n_____\n_____");
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
