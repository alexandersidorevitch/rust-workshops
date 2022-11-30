use std::sync::{Arc, mpsc, Mutex};
use std::thread;

use {
    clap::Parser,
    serde_yaml::Value,
    std::{
        collections::HashMap,
        fs::File,
        path::{Path, PathBuf},
        time::Instant,
    },
};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// Path to yaml directory
    #[arg(short, long)]
    path: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let path = std::fs::canonicalize(Path::new(&args.path))?;

    let files = HashMap::<PathBuf, Value>::new();
    let mut mutex_file = Arc::new(Mutex::new(files));
    let now = Instant::now();

    let (tx, rx) = mpsc::channel::<>();
    let mutex_file_copy = Arc::clone(&mutex_file);

    let rh = thread::spawn(move || {
        loop {
            if let Ok((path, value)) = rx.recv() {
                mutex_file_copy.lock().unwrap().insert(path, value);
            } else {
                break;
            }
        }
    });
    let wh = thread::spawn(move || {
        let mut handlers = vec![];

        for file in std::fs::read_dir(path).unwrap() {
            let file = file.unwrap().path();

            if file
                .extension()
                .map(|ext| ext != "yaml" && ext != "yml")
                .unwrap_or(true)
            {
                continue;
            }
            let tx_copy = tx.clone();
            let handler = thread::spawn(move || {
                if let Ok(reader) = File::open(&file) {
                    let value = serde_yaml::from_reader(reader).unwrap();
                    tx_copy.send((file, value));
                }
            });
            handlers.push(handler);
        }
        handlers.into_iter().for_each(|h| {
            h.join();
        });
    });

    rh.join();
    wh.join();

    println!(
        "{} files read in {} secs",
        mutex_file.lock().unwrap().len(),
        now.elapsed().as_secs_f64()
    );

    Ok(())
}
