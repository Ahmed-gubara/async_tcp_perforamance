# async_tcp_perforamance

A simple benchmark for tcp handling of utf8 strings and find longest string with no duplicated characters.


# Test Configurations
```
no_of_clients 20
```
```
no_of_clients 100
```
```
no_of_clients 200
```
```
no_of_clients 400
```

# Results

### Client 
>`made with Rust`

Execute with:
```
cargo run --manifest-path rust_tcp/Cargo.toml --release  [no_of_clients] [no_of_clients]
```
or (iterations will be 2_000_000/no_of_clients)
```
cargo run --manifest-path rust_tcp/Cargo.toml --release  [no_of_clients] 
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
#### 20
    toke 32.3207388s , 61879 op/s
    min 73.5µs. max 10.5742ms. avg 322.558µs
#### 100
    toke 33.0217504s , 60566 op/s
    min 84µs. max 42.2135ms. avg 1.637044ms
#### 200
    toke 34.6441539s , 57729 op/s
    min 84.7µs. max 93.5749ms. avg 3.417787ms
#### 400
    toke 36.105499s , 55393 op/s
    min 88.6µs. max 212.8036ms. avg 7.034237ms
#### 800
    toke 33.5439239s , 59623 op/s
    min 95.9µs. max 382.792ms. avg 12.759247ms
#### 1600
    toke 32.6184292s , 61315 op/s
    min 116.7µs. max 746.1308ms. avg 24.073898ms
#### 3200
    toke 31.8530088s , 62788 op/s
    min 171.8µs. max 1.4585892s. avg 45.88679ms
#### 6400
    toke 30.9174251s , 64584 op/s
    min 958.6µs. max 2.3007664s. avg 86.579802ms
    
## **C#**
Execute with:
```
dotnet run --project DotnetTcp --confiuration release
```
#### 20
    toke 42.9562584s , 46558 op/s
    min 93.8µs. max 15.9735ms. avg 428.732µs
#### 100
    toke 37.4004697s , 53475 op/s
    min 175µs. max 24.4231ms. avg 1.867039ms
#### 200
    toke 38.8932215s , 51422 op/s
    min 710.1µs. max 34.6522ms. avg 3.817683ms
#### 400
    toke 35.3399902s , 56593 op/s
    min 1.2658ms. max 179.1333ms. avg 6.859234ms
#### 800
    toke 39.8216082s , 50223 op/s
    min 4.9462ms. max 169.1168ms. avg 15.690744ms
#### 1600
    toke 44.1282371s , 45322 op/s
    min 8.2425ms. max 156.7069ms. avg 33.518023ms
#### 3200
    toke 53.3028334s , 37521 op/s
    min 27.5547ms. max 295.6438ms. avg 75.07566ms
#### 6400
    toke 53.7923761s , 37120 op/s
    min 126.9605ms. max 359.694ms. avg 157.63542ms

## **Rust**
Execute with:
```
cargo run --manifest-path rust_tcp/Cargo.toml --release 
```
#### 20
    toke 35.381161s , 56527 op/s
    min 49.7µs. max 5.43ms. avg 353.35µs
#### 100
    toke 23.9695553s , 83439 op/s
    min 81.4µs. max 25.4344ms. avg 1.195758ms
#### 200
    toke 21.9012088s , 91319 op/s
    min 93.8µs. max 40.7531ms. avg 2.180891ms
#### 400
    toke 23.7767335s , 84115 op/s
    min 135.4µs. max 48.0091ms. avg 4.732554ms
#### 800
    toke 22.3107641s , 89642 op/s
    min 219µs. max 60.5343ms. avg 8.853582ms
#### 1600
    toke 22.1278196s , 90383 op/s
    min 685.8µs. max 130.6104ms. avg 17.134663ms
#### 3200
    toke 21.7161339s , 92097 op/s
    min 1.911ms. max 238.1492ms. avg 33.780156ms
#### 6400
    toke 23.3318143s , 85582 op/s
    min 11.1793ms. max 421.7852ms. avg 70.102181ms

| -- | 20 | 100 | 200 | 400 | 800 | 1600 | 3200 |
| ------ | ------ | ------ | ------ | ------ | ------ | ------ | ------ |
| Go | 61879 op/s | 60566 op/s | 57729 op/s | 55393 op/s | 59623 op/s | 61315 op/s | 62788 op/s |
| C# | 46558 op/s | 53475 op/s | 51422 op/s | 56593 op/s | 50223 op/s | 45322 op/s | 37521 op/s |
| Rust | 56527 op/s | 83439 op/s | 91319 op/s | 84115 op/s | 89642 op/s | 90383 op/s | 92097 op/s |



