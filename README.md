# rust-downloadr
download arweave tx-data in rust by splitting it up in chunks of data and writing the data to file

If you get UnexpectedEof error and you know tx_id is correct then just try again.

### Some "benchmarking"

| Method        | Release build                |
|:------------: | :---------------------------:|
| Bufwriter(1)  | 112s - 11s - 10s - 12s - 11s |
| Fs::write(1)  | 13s - 30s - 99s - 13s - 10s |
| Bufwriter(2)  | 10s - 10s - 10s - 35s - 10s |
| Fs::write(2)  | 86s - 10s - 13s - 108s - 10s |

This data does not say much since its only 5 iterations per method. 

Method 1 starts with final tx in a chunk, method 2 starts with first tx in a chunk.

Bufwriter writes to file once data is fetched from API in the loop. 
Fs::write stores data in a vector and writes to file once loop is finished.

"benchmarking" was done by fetching tx: BfOtg-A5EP8RmPQwa7V-fRORQFdUlAM6OYARwori_qE, writing to file "asd.txt" and priting elapsed time to run program.

### Run:

***$ cargo run --transaction <tx_id> --output <file_name>***


### Example:

***$ cargo run -- --transaction "K9u_6E9tO8yr6Jx1D_lHz2tRhSIThPrXpmgazFw8BWI" --output "./asd.txt"***

***For a better performance build with release flag:***

***$ cargo run --release -- --transaction "K9u_6E9tO8yr6Jx1D_lHz2tRhSIThPrXpmgazFw8BWI" --output "./asd.txt"***
