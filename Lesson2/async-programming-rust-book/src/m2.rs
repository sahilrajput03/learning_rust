#![allow(dead_code, unused)]
//? LEARN: #![] means crate level attricute, whereas #[] means attribute on a particular struct, module, function, etc. Source: https://stackoverflow.com/a/25877389/10012446
// Source: https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html

use futures::executor::block_on; // block_no blocks the current thread.

async fn learn_song() -> Song {
    println!("learning...");
    Song("Hey mama!".to_string())
}

async fn sing_song(song: Song) {
    println!("singing...")
}

async fn dance() {
    println!("dancing...")
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await; //? waits till song is learned ~Sahil.
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.

    // In this example, learning the song must happen before singing the song, but both learning and singing can happen at the same time as dancing. If we used block_on(learn_song()) rather than learn_song().await in learn_and_sing, the thread wouldn't be able to do anything else while learn_song was running. This would make it impossible to dance at the same time. By .await-ing the learn_song future, we allow other tasks to take over the current thread if learn_song is blocked. This makes it possible to run multiple futures to completion concurrently on the same thread.
    futures::join!(f1, f2); //? f1 and f2 are run concurrent (parallel).
}

pub fn main() {
    println!("EXECUTING module m2::");
    block_on(async_main());
}

struct Song(String);
