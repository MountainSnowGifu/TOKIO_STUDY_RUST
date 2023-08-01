use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{fs, io, path::Path};
type ExcelVec = Arc<Mutex<Vec<PathBuf>>>;
type ExcelCount = Arc<Mutex<i32>>;

fn main() {
    let v = read_dir_entries("/Users/akira/Desktop/kaori/動画")
        .unwrap()
        .into_iter()
        .filter(|x| x.to_str().unwrap().contains("."))
        .collect::<Vec<PathBuf>>();

    println!("{:?}", v.len());
    thread::sleep(Duration::from_secs(3));

    let list = Arc::new(Mutex::new(v));
    let counter = Arc::new(Mutex::new(0));

    let task1_list = Arc::clone(&list);
    let task1_counter = Arc::clone(&counter);

    let task1 = thread::spawn(move || {
        println!("loop");
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
        println!("loop");
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

    task1.join().unwrap();
    task2.join().unwrap();
    // let (res1, res2): (i32, String) = tokio::join!(task1, task2);
    // println!("{} {}", res1, res2);

    println!("{:?}", counter.lock().unwrap());
}

fn read_dir_entries<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    Ok(entries)
}

fn test(list: &ExcelVec, counter: &ExcelCount) {
    let (tx0, rx) = mpsc::channel();
    let tx1 = tx0.clone();
    let tx2 = tx0.clone();

    let v0 = Arc::clone(&list);
    let v0_counter = Arc::clone(&counter);

    let v0_task = thread::spawn(move || {
        let result = v0.lock().unwrap().pop();

        match result {
            Some(v) => {
                let excel_name = v.to_str().unwrap().to_owned();
                tx0.send(excel_name).unwrap();
                let mut num = v0_counter.lock().unwrap();
                *num += 1;
            }
            None => println!("None"),
        }
    });

    let v1 = Arc::clone(&list);
    let v1_counter = Arc::clone(&counter);

    let v1_task = thread::spawn(move || {
        let result = v1.lock().unwrap().pop();
        match result {
            Some(v) => {
                let excel_name = v.to_str().unwrap().to_owned();
                tx1.send(excel_name).unwrap();
                let mut num = v1_counter.lock().unwrap();
                *num += 1;
            }
            None => println!("None"),
        }
    });

    let v2 = Arc::clone(&list);
    let v2_counter = Arc::clone(&counter);

    let v2_task = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let result = v2.lock().unwrap().pop();
        match result {
            Some(v) => {
                let excel_name = v.to_str().unwrap().to_owned();
                tx2.send(excel_name).unwrap();
                let mut num = v2_counter.lock().unwrap();
                *num += 1;
            }
            None => println!("None"),
        }
    });

    //thread::sleep(Duration::from_secs(1));

    v0_task.join().unwrap();
    v1_task.join().unwrap();
    v2_task.join().unwrap();

    println!("{:?}", list.lock().unwrap());

    for val in rx.iter() {
        println!("recv>{}", val);
    }
}
