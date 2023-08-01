use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::{fs, io, thread};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let v = read_dir_entries("/Users/akira/Desktop/kaori/動画")
        .unwrap()
        .into_iter()
        .filter(|x| x.to_str().unwrap().contains("."))
        .collect::<Vec<PathBuf>>();

    println!("{:?}", v.len());

    let list = Arc::new(Mutex::new(v));
    let counter = Arc::new(Mutex::new(0));

    let task1_list = Arc::clone(&list);
    let task1_counter = Arc::clone(&counter);

    let task1 = thread::spawn(move || {
        let mut count = 0;
        while task1_list.lock().unwrap().len() > 0 {
            let result = task1_list.lock().unwrap().pop();

            match result {
                Some(path) => {
                    println!("{:?}", path);
                    let mut num = task1_counter.lock().unwrap();
                    *num += 1;
                }
                None => println!("None"),
            }
            count += 1;
        }
        count
    });

    let task2_list = Arc::clone(&list);
    let task2_counter = Arc::clone(&counter);

    let task2 = thread::spawn(move || {
        let mut count = 0;
        while task2_list.lock().unwrap().len() > 0 {
            let result = task2_list.lock().unwrap().pop();

            match result {
                Some(path) => {
                    println!("{:?}", path);
                    let mut num = task2_counter.lock().unwrap();
                    *num += 1;
                }
                None => println!("None"),
            }
            count += 1;
        }
        count
    });

    let t = task1.join().unwrap();
    let t2 = task2.join().unwrap();
    println!("{} + {} = {}", t, t2, counter.lock().unwrap());
}

fn read_dir_entries<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    Ok(entries)
}
