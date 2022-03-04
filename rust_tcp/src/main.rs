#![allow(unused, unused_imports)]
// pub mod main_client;
// pub mod main_server;

pub mod unique_substring;
// use bumpalo::Bump;

use async_std::{net::TcpListener, prelude::*, task};
use dialoguer::{theme::ColorfulTheme, Select};
use unique_substring::find_longest_unique_substring;

use crate::unique_substring::find_longest_unique_substring_alt;
use std::{
    alloc::System,
    borrow::BorrowMut,
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    env::{args, Args},
    error::Error,
    fmt::{Debug, Display},
    io::{stdin, stdout, Write},
    ops::{Add, DerefMut, Div, Mul},
    string,
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
    vec,
};

// #[global_allocator]
// static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
struct Dur {
    max: Duration,
    min: Duration,
    total: Duration,
    rounds: u128,
    ignored: u128,
}
impl Display for Dur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let avg = self.avarage();

        write!(
            f,
            "avg:{},max:{},min:{}",
            avg,
            self.max.as_nanos(),
            self.min.as_nanos()
        )
    }
}
impl Dur {
    fn new() -> Self {
        Self {
            max: Duration::ZERO,
            min: Duration::MAX,
            total: Duration::ZERO,
            rounds: 0,
            ignored: 50,
        }
    }
    fn add(&mut self, duration: Duration) {
        self.rounds += 1;
        if self.rounds < self.ignored {
            return;
        }
        if duration > self.max {
            self.max = duration;
        }
        if duration < self.min {
            self.min = duration;
        }
        self.total += duration;
    }
    fn avarage(&self) -> u128 {
        self.total.as_nanos() / (self.rounds - self.ignored)
    }
    fn run<T>(&mut self, call: fn() -> T) {
        self.rounds += 1;
        let start = tokio::time::Instant::now();
        let x = call();
        let end = tokio::time::Instant::now();
        if self.rounds < 50000 {
            return;
        }
        let duration = end.duration_since(start);
        if duration > self.max {
            self.max = duration;
        }
        if duration < self.min {
            self.min = duration;
        }
        self.total += duration;
    }
}

async fn main_async_std() -> Result<(), Box<dyn Error>> {
    use async_std::{
        io::{BufRead, BufReader, BufWriter},
        net::{TcpListener, TcpStream},
        task,
    };
    use std::time::Instant;
    let args = args().collect::<Vec<_>>();
    let arg = args.len();
    // let mut stdin = stdin();
    if arg > 1 {
        let no_of_clients = args[1].parse().unwrap();
        let iterations = args
            .get(2)
            .map(|f| f.parse().expect("invalid number for iterations"))
            .unwrap_or_else(|| 2_000_000_usize.div(no_of_clients));
        let mut tasks = Vec::with_capacity(no_of_clients);
        let starting = Instant::now();
        for thread_id in 1..=tasks.capacity() {
            let tc = tasks.capacity();
            let join = task::spawn(async move {
                let mut timer = Instant::now();

                // println!("len {}", s.len());
                let mut buf = String::with_capacity(1024);
                let mut tcp = TcpStream::connect("127.0.0.1:8080")
                    .await
                    .expect("Can't connect");
                tcp.set_nodelay(true).unwrap();
                let mut writer = BufWriter::new(tcp.clone());
                let mut reader = BufReader::new(tcp.clone());
                // let mut writer=BufWriter::new(tcp_write);
                // const INTERVAL: u32 = 1_000;
                for x in 0..iterations {
                    writer.write_all(S.as_bytes()).await.unwrap();
                    writer.flush().await.unwrap();
                    // writeln!(writer, "{}", s).expect("Can't write");
                    // tcp.flush().expect("flushheee!");
                    buf.clear();
                    let c = reader.read_line(&mut buf).await.expect("Can't read");
                    // dbg!(c);
                    if x % iterations == 0 {
                        let elapsed = timer.elapsed();
                        // println!(
                        //     "T:{:2},{:7} , {} ms, {} microSecond/op, {} op/s   <{}>",
                        //     thread_id,
                        //     x,
                        //     elapsed.as_millis(),
                        //     elapsed.as_micros() / INTERVAL as u128,
                        //     (INTERVAL as f64 / elapsed.as_secs_f64()) as u128,
                        //     (INTERVAL as f64 / elapsed.as_secs_f64()) as u128 * tc as u128
                        // );
                        timer = Instant::now();
                    }

                    if (c != 0 && x == 0) {
                        // println!("received [{}]", buf);
                        // stdout().flush();
                    }
                }

                let end = Instant::now();
                // println!(
                //     "T:{},avg:{:?}",
                //     thread_id,
                //     end.duration_since(begin) / 4_000
                // );
            });
            tasks.push(join);
            // threads.push(thread);
        }
        for task in tasks {
            task.await;
        }

        let x = ((no_of_clients * iterations as usize) as f64 / starting.elapsed().as_secs_f64())
            as usize;
        println!("toke {:?} , {} op/s", starting.elapsed(), x);
    }
    if arg == 1 {
        let tcp = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        let no_clients = Arc::new(AtomicUsize::new(0));
        println!("Waiting for connection");
        loop {
            let (mut stream, _) = tcp.accept().await.unwrap();
            // let stream = connection;
            let add = stream.peer_addr().unwrap();
            // stream.set_nodelay(false).unwrap();
            use std::sync::atomic::Ordering as AOrdering;
            let arc = no_clients.clone();
            let c_count = arc.fetch_add(1, AOrdering::SeqCst);
            // dbg!("xxxx");
            task::spawn(async move {
                // let mut c_count = no_clients.lock().await;
                // *c_count += 1_usize;
                // println!("{} Connected , count {}", add, c_count);
                // drop(c_count);
                // let bump = Bump::new();
                let mut writer = BufWriter::new(stream.clone());
                // let mut reader = BufReader::new(stream.clone());
                // let (mut reader, mut writer) = stream.split();
                let mut x = Duration::ZERO;
                let mut count = 0;
                // let mut buf = String::with_capacity(10240);
                let mut buf = vec![0; 1024];
                let mut reader = BufReader::with_capacity(10240, stream); // let mut writer = BufWriter::new(writer);// '\u{ffff}'
                loop {
                    buf.clear();
                    // let mut char_indexes = Box::new(vec![None; 'z' as usize]);
                    let begin = Instant::now();

                    match reader.read_until(b'\n', &mut buf).await.unwrap_or_default() {
                        0 => {
                            // println!("{} nanos", (x.as_nanos()) / (count));
                            break;
                        }
                        l => {
                            let end = begin.elapsed();
                            x += end;
                            let st = String::from_utf8_lossy(&buf[..l]);
                            // println!("{} : {}",add,l);
                            // dbg!(l);
                            let vec = st.trim().chars().collect::<Vec<char>>();
                            // let max=(vec.iter().max().map(|f|*f as u16).unwrap_or(0)+1) as usize;
                            // char_indexes.fill(None);
                            // if char_indexes.len()< max{ char_indexes.resize(max  , None)}
                            // char_indexes.insert(max , None);
                            // Box::new([Option::<usize>::None; (char::MAX as u16) as usize]);

                            let result = find_longest_unique_substring(&vec); //, char_indexes.deref_mut(), 0);

                            // let result = &st[..9];
                            // result.to_owned()
                            // println!("{} : {}>>>>",add,result.len());
                            let w = format!("{}\r\n", result);
                            // println!("{} : {}>>>>",add,w);

                            writer.write_all(w.as_bytes()).await.unwrap();
                            // writeln!(writer, "{:?}", result).expect("Failed to write to stream");
                            writer.flush().await.unwrap();
                            count += 1;
                        }
                    };
                }

                let c_count = arc.fetch_sub(1, AOrdering::SeqCst);
                // println!("{} disconnected , count {}", add, c_count);
            });
        }
    }
    Ok(())
}

fn main() {
    let runtimes = ["Tokio", "Async-std"];

    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose runtime:")
        .items(&runtimes)
        .interact()
        .unwrap();
    let use_tokio = selected == 0;
    if !use_tokio {
        task::block_on(main_async_std()).unwrap()
    } else if true {
        // let runtime = tokio::runtime::Builder::new_multi_thread()
        //     .worker_threads(15)
        //     .enable_all()
        //     .build()
        //     .unwrap();
        match args().count() {
            1 => {
                let mut tok = tokio::runtime::Builder::new_multi_thread();
                // let cpu = dialoguer::Input::<usize>::new()
                //     .with_prompt("cpu cores")
                //     .default(0)
                //     .interact()
                //     .unwrap();

                tok.enable_all().build().unwrap()

                // .worker_threads(32)
            }
            _ => tokio::runtime::Builder::new_multi_thread()
                // .worker_threads(32)
                .enable_all()
                .build()
                .unwrap(),
        }
        .block_on(main_tokio())
    }
}
// const NO_OF_TASKS: usize = 20;
// const TOTAL: u32 = 100_000;
const INTERVAL: u32 = 100_000;
const S:&str= "You might encounter someone with a computer science background preferring to use the term hash table. Perl and Ruby strip that off and call them hashes. Lua does the opposite and uses the term table. Many communities name the structure after a metaphor, such as a dictionary (one term is being associated with a “definition”) or a map (programmers, following mathematicians, are mapping from one value to another). Other communities prefer naming based on the role that the structure plays. PHP describes them as associative arrays. JavaScript’s objects tend to be implemented as a key/value pair collection and so generic term object suffices. Static languages tend to name them according to how they are implemented. C++ and Java distinguish between a hash map and a tree map.\
Rust uses the terms HashMap and BTreeMap to define two i and “associative array” refer to the abstract data type. “Hash table” refers to associative arrays implemented with a hash table. HashMaprefers to Rust’s implementation of hash tables.\
You might encounter someone with a computer science bacmplementations of the same abstract data type. Rust is closest to C++ and Java in this regard.\
\r\n";
// #[tokio::main]
pub async fn main_tokio() {
    use tokio::{
        io::{
            AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader,
            BufWriter, WriteHalf,
        },
        net::{TcpListener, TcpStream},
        sync::Mutex,
        time::Instant,
    };
    let args = args().collect::<Vec<_>>();
    let arg = args.len();
    println!("{:?}", args);
    // let mut stdin = stdin();
    if arg > 1 {
        let no_of_clients = args[1].parse().unwrap();
        let iterations = args
            .get(2)
            .map(|f| f.parse().expect("invalid number for iterations"))
            .unwrap_or_else(|| 2_000_000_usize.div(no_of_clients));
        let mut tasks = Vec::with_capacity(no_of_clients);
        let starting = Instant::now();
        for thread_id in 1..=tasks.capacity() {
            let tc = tasks.capacity();
            let join = tokio::spawn(async move {
                let begin = Instant::now();
                let mut timer = Instant::now();
                let mut dur = Dur::new();
                // println!("len {}", s.len());
                // let mut buf = String::with_capacity(1024);
                let mut buf = vec![0; 1024];
                let mut tcp = TcpStream::connect("127.0.0.1:8080")
                    .await
                    .expect("Can't connect");
                tcp.set_nodelay(true).unwrap();
                // let mut writer = BufWriter::new(tcp.try_clone());
                let (reader, mut writer) = tcp.split();
                let mut reader = BufReader::new(reader);
                // let mut writer = BufWriter::new(writer);
                // let mut writer=BufWriter::new(tcp_write);
                for x in 0..iterations {
                    let mut t = Instant::now();
                    writer.write_all(S.as_bytes()).await.unwrap();
                    buf.clear();
                    let c = reader
                        .read_until(b'\n', &mut buf)
                        .await
                        .expect("Can't read");
                    dur.add(t.elapsed());

                    // dbg!(c);
                    if x % iterations == 0 {
                        let elapsed = timer.elapsed();
                        // println!(
                        //     "T:{:2},{:7} , {} ms, {} microSecond/op, {} op/s   <{}>",
                        //     thread_id,
                        //     x,
                        //     elapsed.as_millis(),
                        //     elapsed.as_micros() / INTERVAL as u128,
                        //     (INTERVAL as f64 / elapsed.as_secs_f64()) as u128,
                        //     (INTERVAL as f64 / elapsed.as_secs_f64()) as u128 * tc as u128
                        // );
                        timer = Instant::now();
                    }

                    if (c != 0 && x == 0) {
                        // println!("received [{}]", String::from_utf8_lossy(&buf));
                        // stdout().flush();
                    }
                }

                let end = Instant::now();
                // println!(
                //     "T:{},avg:{:?}",
                //     thread_id,
                //     end.duration_since(begin) / 4_000
                // );
                dur
            });
            tasks.push(join);
            // threads.push(thread);
        }
        let mut _min = Duration::ZERO;
        let mut _max = Duration::ZERO;
        let mut _total = 0u128;
        for task in tasks {
            let dur = task.await.unwrap();
            if dur.min > _min {
                _min = dur.min;
            }
            if dur.max > _max {
                _max = dur.max;
            }
            _total += dur.avarage();
        }
        _total /= no_of_clients as u128;
        let x = ((no_of_clients * iterations as usize) as f64 / starting.elapsed().as_secs_f64())
            as usize;
        println!("toke {:?} , {} op/s", starting.elapsed(), x);
        println!(
            "min {:?}. max {:?}. avg {:?}",
            _min,
            _max,
            Duration::from_nanos(_total.try_into().unwrap())
        );
    }
    if arg == 1 {
        let tcp = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        let no_clients = Arc::new(AtomicUsize::new(0));
        println!("Waiting for connection");
        loop {
            let (mut stream, _) = tcp.accept().await.unwrap();
            // let stream = connection;
            let add = stream.peer_addr().unwrap();
            // stream.set_nodelay(false).unwrap();
            use std::sync::atomic::Ordering as AOrdering;
            let arc = no_clients.clone();
            let c_count = arc.fetch_add(1, AOrdering::SeqCst);
            // dbg!("xxxx");
            tokio::spawn(async move {
                // let mut c_count = no_clients.lock().await;
                // *c_count += 1_usize;
                // println!("{} Connected , count {}", add, c_count);
                // drop(c_count);
                // let bump = Bump::new();

                let (mut reader, mut writer) = stream.split();
                let mut x = Duration::ZERO;
                let mut count = 0;
                // let mut buf = String::with_capacity(10240);
                let mut buf = vec![0; 1024];
                let mut reader = BufReader::new(reader); // let mut writer = BufWriter::new(writer);// '\u{ffff}'
                loop {
                    buf.clear();
                    // let mut char_indexes = Box::new(vec![None; 'z' as usize]);
                    let begin = Instant::now();

                    match reader.read_until(b'\n', &mut buf).await.unwrap_or_default() {
                        0 => {
                            // println!("{} nanos", (x.as_nanos()) / (count));
                            break;
                        }
                        l => {
                            let end = begin.elapsed();
                            x += end;
                            let st = String::from_utf8_lossy(&buf[..l]);
                            // println!("{} : {}",add,l);
                            // dbg!(l);
                            let vec = st.trim().chars().collect::<Vec<char>>();
                            // let max=(vec.iter().max().map(|f|*f as u16).unwrap_or(0)+1) as usize;
                            // char_indexes.fill(None);
                            // if char_indexes.len()< max{ char_indexes.resize(max  , None)}
                            // char_indexes.insert(max , None);
                            // Box::new([Option::<usize>::None; (char::MAX as u16) as usize]);
                            let result = { find_longest_unique_substring(&vec) };

                            // let result = find_longest_unique_substring(&vec); //, char_indexes.deref_mut(), 0);

                            // let result = &st[..9];
                            // result.to_owned()
                            // println!("{} : {}>>>>",add,result.len());
                            let w = format!("{}\n", result);
                            // println!("{} : {}>>>>",add,w);
                            // writer.write_all(result.as_bytes()).await.unwrap();
                            // writer.write_all(b"\r\n").await.unwrap();
                            writer.write_all(w.as_bytes()).await.unwrap();
                            // writeln!(writer, "{:?}", result).expect("Failed to write to stream");
                            writer.flush().await.unwrap();
                            // count += 1;
                            let _ = result;
                        }
                    };
                }

                let c_count = arc.fetch_sub(1, AOrdering::SeqCst);
                // println!("{} disconnected , count {}", add, c_count);
            });
        }
    }
}
