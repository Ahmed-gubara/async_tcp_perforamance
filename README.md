# async_tcp_perforamance


const NO_OF_TASKS: usize = 100;
const TOTAL: u32 = 10_000;

GO
go run gotcp/main.go 
toke 16.4425877s , 60817 op/s
min 2.8µs. max 1.7738ms. avg 5.184µs

C#
dotnet run --project DotnetTcp --confiuration release
toke 19.1821173s , 52131 op/s
min 2.8µs. max 3.9366ms. avg 10.06µs

Rust
cargo run --manifest-path rust_tcp/Cargo.toml --release server
toke 11.8413077s , 84450 op/s
min 2.8µs. max 1.6872ms. avg 4.69µs



to run client
cargo run --manifest-path rust_tcp/Cargo.toml --release 


