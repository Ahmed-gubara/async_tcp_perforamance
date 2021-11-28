#![allow(unused, unused_imports)]
pub mod bench;
// pub mod main_client;
// pub mod main_server;
pub mod parser;
pub mod point;
pub mod unique_substring;
// use bumpalo::Bump;

use async_std::{net::TcpListener, prelude::*, task};
use unique_substring::find_longest_unique_substring;

use crate::{
    parser::valueparser::ReadFromTerminal, point::Point,
    unique_substring::find_longest_unique_substring_alt,
};
use std::{
    alloc::System,
    borrow::BorrowMut,
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    env::{args, Args},
    error::Error,
    fmt::{Debug, Display},
    io::{stdin, stdout, Write},
    ops::{Add, DerefMut, Mul},
    string,
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
    vec,
};

use jemallocator::Jemalloc;
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;
struct Dur {
    max: Duration,
    min: Duration,
    total: Duration,
    rounds: u128,
}
impl Display for Dur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let avg = self.total.as_nanos() / (self.rounds - 50000);

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
        }
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
    let arg = args().nth(1);
    // let mut stdin = stdin();
    if arg.is_none() {
        let mut tasks = Vec::with_capacity(NO_OF_TASKS);
        let starting = Instant::now();
        for thread_id in 1..=tasks.capacity() {
            let tc = tasks.capacity();
            let join = task::spawn(async move {
                let begin = Instant::now();
                let mut timer = Instant::now();

                {
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
                    for x in 0..TOTAL {
                        writer.write_all(S.as_bytes()).await.unwrap();
                        writer.flush().await.unwrap();
                        // writeln!(writer, "{}", s).expect("Can't write");
                        // tcp.flush().expect("flushheee!");
                        buf.clear();
                        let c = reader.read_line(&mut buf).await.expect("Can't read");
                        // dbg!(c);
                        if x % INTERVAL == 0 {
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

        let x = ((NO_OF_TASKS * TOTAL as usize) as f64 / starting.elapsed().as_secs_f64()) as usize;
        println!("toke {:?} , {} op/s", starting.elapsed(), x);
    }
    if arg.is_some() {
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

                            let result = result.iter().collect::<String>();

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
    let use_tokio = true;
    if !use_tokio {
        task::block_on(main_async_std()).unwrap()
    } else if true {
        // let runtime = tokio::runtime::Builder::new_multi_thread()
        //     .worker_threads(15)
        //     .enable_all()
        //     .build()
        //     .unwrap();
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap()
            .block_on(main_tokio())
    }
}
const NO_OF_TASKS: usize = 10;
const INTERVAL: u32 = 100_000;
const TOTAL: u32 = 100_000;
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
    let arg = args().nth(1);
    // let mut stdin = stdin();
    if arg.is_none() {
        let mut tasks = Vec::with_capacity(NO_OF_TASKS);
        let starting = Instant::now();
        for thread_id in 1..=tasks.capacity() {
            let tc = tasks.capacity();
            let join = tokio::spawn(async move {
                let begin = Instant::now();
                let mut timer = Instant::now();

                {
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
                    for x in 0..TOTAL {
                        writer.write_all(S.as_bytes()).await.unwrap();
                        // writer.flush().await.unwrap();
                        // writeln!(writer, "{}", s).expect("Can't write");
                        // tcp.flush().expect("flushheee!");
                        buf.clear();
                        let c = reader
                            .read_until(b'\n', &mut buf)
                            .await
                            .expect("Can't read");
                        // dbg!(c);
                        if x % INTERVAL == 0 {
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
            task.await.unwrap();
        }
        let x = ((NO_OF_TASKS * TOTAL as usize) as f64 / starting.elapsed().as_secs_f64()) as usize;
        println!("toke {:?} , {} op/s", starting.elapsed(), x);
    }
    if arg.is_some() {
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
                let mut reader = BufReader::with_capacity(10240, reader); // let mut writer = BufWriter::new(writer);// '\u{ffff}'
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

                            let result = result.iter().collect::<String>();

                            // let result = &st[..9];
                            // result.to_owned()
                            // println!("{} : {}>>>>",add,result.len());
                            let w = format!("{}\r\n", result);
                            // println!("{} : {}>>>>",add,w);
                            // writer.write_all(result.as_bytes()).await.unwrap();
                            // writer.write_all(b"\r\n").await.unwrap();
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
}
pub fn main1() {
    let mut duration_val = Dur::new();
    let mut duration_val_u128 = Dur::new();
    let mut duration_ref = Dur::new();
    let mut duration_ref_opt = Dur::new();
    let mut duration_struct = Dur::new();
    use std::time::*;
    const ROUNDS: u128 = u16::MAX as u128 * 2;
    for _x in 0..ROUNDS {
        // println!("{:?}", x);

        duration_val.run(bench::pass_by_value);

        duration_val_u128.run(bench::pass_by_value_u128);

        duration_ref.run(bench::pass_by_ref);

        duration_ref_opt.run(bench::pass_by_value_options);

        duration_struct.run(bench::pass_by_struct);
    }
    println!("val     :{}", duration_val);
    println!("ref_u128:{} ", duration_val_u128);
    println!("ref     :{} ", duration_ref);
    println!("val_opt :{} ", duration_ref_opt);
    println!("struct  :{} ", duration_struct);
}

pub fn main2() -> Result<(), Box<dyn Error>> {
    // arrays
    {
        // this is a (mutable ref) to an array, AKA a slice | length is 1024, initial value is 100_u8
        let buffer_ref_mut: &mut [u8; 1024] = &mut [100_u8; 1024];
        buffer_ref_mut[0] = 1;

        // ----

        // this is a (mutable value) of an array !! this is a value type, not a ref | length is 1024, initial value is 100_u8
        let mut buffer_array_mut: [u8; 1024] = [100_u8; 1024];
        buffer_array_mut[1] = 2;

        // this is a (mutable ref) to the array "buffer_array_mut", AKA a slice
        let buffer_array_asref_mut: &mut [u8; 1024] = &mut buffer_array_mut;
        buffer_array_asref_mut[2] = 3;

        // set array values with value of parameter 'with'
        // clear_array(buffer_array_asref_mut, 2); // !! uncomment me to try

        // set array values with value '5' on parameter 'with' for indexes between (4<= and <10)
        clear_array(&mut buffer_array_asref_mut[4..10], 5); // !! uncomment me to try

        // set array values with defualt value of 'u8 - which is zero' for indexes between (7<= and <10)
        clear_array_defualt(&mut buffer_array_asref_mut[7..10]);

        // create a (immutable ref) to the array, this is known as a Slice
        let slice = &buffer_array_asref_mut[..10];
        println!("-- array values are {:?}", slice);
        print_array_with_index(slice);
    }
    // read from terminal, parse string into values
    {
        let mut read = stdin();
        match read.read_input::<String>("Enter your name") {
            Err(err) => panic!("sorry can't read name {}", err),
            Ok(name) => println!("Hello {}", name),
        }
        let num1 = read.read_input("Enter First Number")?;
        let num2 = read.read_input("Enter Second Number")?;
        // you can use any type for point(like String) but...
        let _point_of_string = Point::new("x".to_string(), "y".to_string());
        //max(&_point_of_string, &_point_of_string);
        let _point_of_vec = Point::new(vec![4, 5, 6], vec![1, 2, 3]);
        // max(&_point_of_vec, &_point_of_vec);
        let _point_of_float64 = Point::new(1.2, 3.4);
        max(&_point_of_float64, &_point_of_float64); // will work

        // max will only work if type is a (PartialOrd + Mul<Output = T> + Copy + Sized)
        let x = max::<Point<u8>>(&num2, &num1);

        dynamic_dispatch_print(&[
            &"val of first number is ",
            &num1,
            &" value of second number is ",
            &num2,
        ]);

        match x {
            Max::Some(big) => println!("The bigger number is {:?}", big), //todo: try write big * 100
            Max::None => println!("The bigger number is unknown!"),
            Max::Equal => println!("They are the same"),
        }
    }
    Ok(())
}

fn max<'a, R>(lhs: &'a R, rhs: &'a R) -> Max<R>
where
    R: PartialOrd + ToOwned<Owned = R>,
{
    match lhs.partial_cmp(rhs) {
        Some(o) => match o {
            Ordering::Less => Max::Some(rhs.to_owned()),
            Ordering::Greater => Max::Some(lhs.to_owned()),
            Ordering::Equal => Max::Equal,
        },
        None => Max::None,
    }
}

pub fn sum<T: Add + Add<Output = R>, R>(rhs: T, lhs: T) -> R {
    lhs.add(rhs)
}

enum Max<T> {
    Some(T),
    Equal,
    None,
}

fn dynamic_dispatch_print(vals: &[&dyn Display]) {
    let x = vals
        .iter()
        .map(|val| val.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", x);
}

pub fn clear_array<T: Clone>(array: &mut [T], with: T) {
    for val in array {
        *val = with.clone();
    }
}
pub fn clear_array_defualt<T: Default>(array: &mut [T]) {
    for val in array {
        *val = T::default();
    }
}

// is the same as
// pub fn print_array_with_index<T: Debug>(array: &[T]) {

pub fn print_array_with_index(array: &[impl Debug]) {
    let p = array
        .iter()
        .enumerate()
        .map(|f| format!("{:?} => {:?}", f.0, f.1))
        .collect::<Vec<_>>();
    println!("{:?}", p);
}
