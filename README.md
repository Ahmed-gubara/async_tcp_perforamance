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
#### Test 4
```rust
const NO_OF_TASKS: usize = 400;
const TOTAL: u32 = 5_000;
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
    toke 32.3207388s , 61879 op/s
    min 73.5µs. max 10.5742ms. avg 322.558µs
#### test 2
    toke 33.0217504s , 60566 op/s
    min 84µs. max 42.2135ms. avg 1.637044ms
#### test 3
    toke 34.6441539s , 57729 op/s
    min 84.7µs. max 93.5749ms. avg 3.417787ms
#### test 4
    toke 36.105499s , 55393 op/s
    min 88.6µs. max 212.8036ms. avg 7.034237ms

## **C#**
Execute with:
```
dotnet run --project DotnetTcp --confiuration release
```
#### Test 1
    toke 42.9562584s , 46558 op/s
    min 93.8µs. max 15.9735ms. avg 428.732µs
#### Test 2
    toke 37.4004697s , 53475 op/s
    min 175µs. max 24.4231ms. avg 1.867039ms
#### Test 3
    toke 38.8932215s , 51422 op/s
    min 710.1µs. max 34.6522ms. avg 3.817683ms
#### Test 4
    toke 35.3399902s , 56593 op/s
    min 1.2658ms. max 179.1333ms. avg 6.859234ms

## **Rust**
Execute with:
```
cargo run --manifest-path rust_tcp/Cargo.toml --release server
```
#### Test 1
    toke 35.381161s , 56527 op/s
    min 49.7µs. max 5.43ms. avg 353.35µs
#### Test 2
    toke 23.9695553s , 83439 op/s
    min 81.4µs. max 25.4344ms. avg 1.195758ms
#### Test 3
    toke 21.9012088s , 91319 op/s
    min 93.8µs. max 40.7531ms. avg 2.180891ms
#### Test 4
    toke 23.7767335s , 84115 op/s
    min 135.4µs. max 48.0091ms. avg 4.732554ms


| -- | Test 1 | Test 2 | Test 3 | Test 4 |
| ------ | ------ | ------ | ------ | ------ |
| Go | 61879 op/s | 60566 op/s | 57729 op/s | 55393 op/s |
| C# | 46558 op/s | 53475 op/s | 51422 op/s | 56593 op/s |
| Rust | 56527 op/s | 83439 op/s | 91319 op/s | 84115 op/s |



