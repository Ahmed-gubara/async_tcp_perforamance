# async_tcp_perforamance


test0
const NO_OF_TASKS: usize = 20;
const TOTAL: u32 = 100_000;

test1
const NO_OF_TASKS: usize = 100;
const TOTAL: u32 = 20_000;

test2
const NO_OF_TASKS: usize = 200;
const TOTAL: u32 = 10_000;

GO
go run gotcp/main.go 
-test0
    toke 33.8627358s , 59061 op/s
    min 2.7µs. max 1.8609ms. avg 5.189µs
-test1
    toke 33.3943256s , 59890 op/s
    min 2.8µs. max 4.5621ms. avg 5.265µs
-test2
    toke 35.2758344s , 56696 op/s
    min 2.9µs. max 9.7254ms. avg 5.413µs

C#
dotnet run --project DotnetTcp --confiuration release
-test0
    toke 41.2984578s , 48427 op/s
    min 2.7µs. max 3.5905ms. avg 11.744µs
-test1
    toke 37.2732647s , 53657 op/s
    min 2.8µs. max 2.7312ms. avg 9.924µs
-test2
    toke 37.9036255s , 52765 op/s
    min 2.9µs. max 3.9085ms. avg 9.769µs

Rust
cargo run --manifest-path rust_tcp/Cargo.toml --release server
-test0
    toke 36.8048513s , 54340 op/s
    min 2.7µs. max 10.1195ms. avg 4.99µs
-test1
    toke 23.6640052s , 84516 op/s
    min 2.8µs. max 2.5449ms. avg 4.669µs
-test2
    toke 21.0595518s , 94968 op/s
    min 2.8µs. max 2.7639ms. avg 4.728µs



to run client
cargo run --manifest-path rust_tcp/Cargo.toml --release 


