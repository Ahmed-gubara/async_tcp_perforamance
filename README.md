# async_tcp_perforamance


go run gotcp/main.go 
toke 22.6606538s for 10 connection , 44129 op/s : GO
toke 226.1565314s for 100 connection , 44217 op/s
\*toke 226.0155691s for 100 connection , 44244 op/s
wsl : toke 16.3521917s , 61153 op/s
2wsl: toke 18.1040504s , 55236 op/s


dotnet run --project DotnetTcp --confiuration release
toke 17.1422447s for 10 connection , 58335 op/s : C#
toke 145.8709757s for 100 connection , 68553 op/s
\*toke 147.3788392s for 100 connection , 67852 op/s
!toke 165.2082136s for 100 connection , 60529 op/s
wsl : toke 16.4258124s , 60879 op/s
2wsl: toke 18.4494671s , 54202 op/s


cargo run --manifest-path rust_tcp/Cargo.toml --release server
toke 20.077528s for 10 connection , 49806 op/s : Rust
toke 201.2566498s for 100 connection , 49687 op/s
\*toke 191.0375479s for 100 connection , 52345 op/s
wsl : toke 12.409343s , 80584 op/s
2wsl: toke 30.1424655s , 33175 op/s


for \* we used 16 worker threads in tokio


to run client
cargo run --manifest-path rust_tcp/Cargo.toml --release 