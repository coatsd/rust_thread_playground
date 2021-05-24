use std::{time, thread};
use rand::Rng;
use std::sync::mpsc;
use waitgroup::WaitGroup;
use futures::executor::block_on;
use futures;

type S<T> = mpsc::Sender<T>;
type R<T> = mpsc::Receiver<T>;

fn sing<'a>(recv: R<&'a str>, w: waitgroup::Worker) {
    let mut rng = rand::thread_rng();
    while let Result::Ok(s) = recv.recv() {
        thread::sleep(time::Duration::from_millis(rng.gen_range(250..501)));
        println!("{}", s);
    }
    drop(w);
}

fn learn_song<'a>(send: S<&'a str>, words: &[&'a str]) {
    for w in words {
        match send.send(w) { 
            Result::Err(_) => {
                println!("{}", "An Error has occurred in send!")
            },
            _ => (),
        }
    }
}

fn dance() {
    let mut rng = rand::thread_rng();
    for i in 1..6 {
        thread::sleep(time::Duration::from_millis(rng.gen_range(250..501)));
        println!("step {}", i);
    }
}

async fn sing_and_dance(s_send: S<&'static str>, s_recv: R<&'static str>, words: &'static[&'static str]) {
    let wg = WaitGroup::new();
    let worker = wg.worker();
    thread::spawn(move || learn_song(s_send, words));
    thread::spawn(move || sing(s_recv, worker));
    dance();
    wg.wait().await;
}

fn main() {
    let (s_send, s_recv) = mpsc::channel::<&str>();
    let words = &["Mary", "had", "a", "little", "lamb"];
    block_on(sing_and_dance(s_send, s_recv, words));
}
