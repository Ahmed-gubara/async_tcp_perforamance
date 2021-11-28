# async_tcp_perforamance

toke 22.6606538s for 10 connection , 44129 op/s : GO
toke 226.1565314s for 100 connection , 44217 op/s
\*toke 226.0155691s for 100 connection , 44244 op/s

toke 17.1422447s for 10 connection , 58335 op/s : C#
toke 145.8709757s for 100 connection , 68553 op/s
\*toke 147.3788392s for 100 connection , 67852 op/s
!toke 165.2082136s for 100 connection , 60529 op/s

toke 20.077528s for 10 connection , 49806 op/s : Rust
toke 201.2566498s for 100 connection , 49687 op/s
\*toke 191.0375479s for 100 connection , 52345 op/s

for \* we used 16 worker threads in tokio
