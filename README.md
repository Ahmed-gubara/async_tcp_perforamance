# async_tcp_perforamance

A simple benchmark for tcp handling of utf8 strings and find longest string with no duplicated characters.


# Test Configurations
#### Test 1
```rust
const NO_OF_TASKS: usize = 20;
const TOTAL: u32 = 100_000;
```
#### Test 2
```rust
const NO_OF_TASKS: usize = 100;
const TOTAL: u32 = 20_000;
```
#### Test 3
```rust
const NO_OF_TASKS: usize = 200;
const TOTAL: u32 = 10_000;
```

# Results

### Client 
>`made with Rust`

Execute with:
```
cargo run --manifest-path rust_tcp/Cargo.toml --release
```
Result descripes
- Total time the Benchmark toke
- number of Operations per seconds
- min time for a request
- max time for a request
- avg time for a request

## **GO**
Execute with:
```
go run gotcp/main.go
```
#### test 1
    toke 33.8627358s , 59061 op/s
    min 2.7µs. max 1.8609ms. avg 5.189µs
#### test 2
    toke 33.3943256s , 59890 op/s
    min 2.8µs. max 4.5621ms. avg 5.265µs
#### test 3
    toke 35.2758344s , 56696 op/s
    min 2.9µs. max 9.7254ms. avg 5.413µs

## **C#**
Execute with:
```
dotnet run --project DotnetTcp --confiuration release
```
#### Test 1
    toke 41.2984578s , 48427 op/s
    min 2.7µs. max 3.5905ms. avg 11.744µs
#### Test 2
    toke 37.2732647s , 53657 op/s
    min 2.8µs. max 2.7312ms. avg 9.924µs
#### Test 3
    toke 37.9036255s , 52765 op/s
    min 2.9µs. max 3.9085ms. avg 9.769µs

## **Rust**
Execute with:
```
cargo run --manifest-path rust_tcp/Cargo.toml --release server
```
#### Test 1
    toke 36.8048513s , 54340 op/s
    min 2.7µs. max 10.1195ms. avg 4.99µs
#### Test 2
    toke 23.6640052s , 84516 op/s
    min 2.8µs. max 2.5449ms. avg 4.669µs
#### Test 3
    toke 21.0595518s , 94968 op/s
    min 2.8µs. max 2.7639ms. avg 4.728µs


| -- | Test 1 | Test 2 | Test 3 |
| ------ | ------ | ------ | ------ |
| Go | 59061 op/s | 59890 op/s | 56696 op/s |
| C# | 48427 op/s | 53657 op/s | 52765 op/s |
| Rust | 54340 op/s | 84516 op/s | 94968 op/s |



