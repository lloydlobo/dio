//! [`server`] implements server side operations.

use crate::{
    db::{Fact, Principle},
    LEN_FACTS, LEN_PRINCIPLES,
};
use anyhow::{Error, Result};
use std::collections::HashMap;

/// `fetcher` is a utility function to fetch with `GET` Method with `reqwest`.
///
/// # Errors
///
/// This function will return an error if it fails to fetch data from the server.
async fn fetcher(token: &str) -> Result<String, Error> {
    const PORT: &str = "5000";
    let url = format!("http://localhost:{PORT}{token}");
    Ok(reqwest::get(url).await?.text().await?)
}

// ----------------------------------------------------------------------------

/// Returns `HashMap` of server data response struct.
///
/// # Errors
///
/// This function will return an error if:
///
/// * Fails to fetch data from the server.
/// * fails to parse response string to json struct of type `Fact` or `Principle`.
pub async fn get_server_data() -> Result<(HashMap<String, String>, HashMap<String, String>), Error>
{
    let mut facts = HashMap::<String, String>::new();
    let mut principles = HashMap::<String, String>::new();

    for i in 1..=LEN_FACTS {
        let fetched: String = fetcher(&format!("/facts/{i}")).await?;
        let deserialized: Fact = serde_json::from_str(&fetched)?;
        facts.insert(deserialized.id.to_string(), deserialized.title);
    }
    for i in 1..=LEN_PRINCIPLES {
        let fetched: String = fetcher(&format!("/principles/{i}")).await?;
        let deserialized: Principle = serde_json::from_str(&fetched)?;
        principles.insert(deserialized.id.to_string(), deserialized.title);
    }

    Ok((facts, principles))
}

mod concurrency {
    use rayon::prelude::*;
    use tokio::sync::oneshot;

    /// We will use the sum of a large list as an example of an expensive computation, but note that
    /// in practice, unless the array is very very large, just computing a sum is probably cheap
    /// enough that you can just do it directly in Tokio.
    ///
    /// The main danger of using rayon is that you must be careful not to block the thread while
    /// waiting for rayon to complete. To do this, combine rayon::spawn with tokio::sync::oneshot
    /// like this:
    ///
    /// [Reference](https://ryhl.io/blog/async-what-is-blocking/)
    ///
    /// # Panics
    ///
    /// Panics if .
    pub async fn parallel_sums(nums: Vec<i32>) -> i32 {
        let (send, recv): (oneshot::Sender<i32>, oneshot::Receiver<i32>) = oneshot::channel();

        // Spawn a task on rayon.
        rayon::spawn(move || {
            // Perform an expensive computation.
            let mut sum = 0;
            for num in nums {
                sum += num;
            }

            // Send the result back to Tokio.
            let _ = send.send(sum);
        });

        // Wait for the rayon task.
        recv.await.expect("Panic in rayon::spawn")
    }

    /// This uses the rayon thread pool to run the expensive operation. Be aware that the above example
    /// uses only one thread in the rayon thread pool per call to parallel_sum. This makes sense if you
    /// have many calls to parallel_sum in your application, but it is also possible to use rayon's
    /// parallel iterators to compute the sum on several threads:
    ///
    /// [Reference](https://ryhl.io/blog/async-what-is-blocking/)
    pub async fn parallel_sums_par(nums: Vec<i32>) -> i32 {
        let (send, recv): (oneshot::Sender<i32>, oneshot::Receiver<i32>) = oneshot::channel();

        // Spawn a task on rayon.
        rayon::spawn(move || {
            // Perform an expensive computation.
            // Compute the sum on multiple threads.
            let sum = nums.par_iter().sum();

            // Send the result back to Tokio.
            let _ = send.send(sum);
        });

        // Wait for the rayon task.
        recv.await.expect("Panic in rayon::spawn")
    }
}
