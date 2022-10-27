# rust-downloadr
download arweave tx-data in rust by splitting it up in chunks of data appending to a vector and printing to file

If you get UnexpectedEof error and you know tx_id is correct then just try again.

### Some "benchmarking"

| Write         | Debug build                   | Release build                |
|:------------: |:-----------------------------:| :---------------------------:|
| Bufwriter(1)  | 134s - 39s - 37s - 192s - 36s | 125s - 33s - 35s - 35s - 34s |
| Fs::write(1)  | 161s - 149s - 36s - 36s - 127s| 123s - 33s - 35s - 34s - 33s |
| Bufwriter(2)  | 134s - 39s - 37s- 36s - 84s   | 117s - 35s - 35s - 35s - 35s |
| Fs::write(2)  | 70s - 37s - 35s - 85s - 34s   | 109s - 33s - 36s - 35s - 33s |

This data does not say much its only 5 iterations per method. 
Method 1 starts with final tx in a chunk, method 2 starts with first tx in a chunk.
Bufwriter writes to file once data is fetched from API in the loop. 
Fs::write stores data in a vector and writes to file once loop is finished.
"benchmarking" was done by fetching tx: BfOtg-A5EP8RmPQwa7V-fRORQFdUlAM6OYARwori_qE, writing to file "asd.txt" and priting elapsed time to run program.

### Run:

***$ cargo run --transaction <tx_id> --output <file_name>***


### Example:

***$ cargo run -- --transaction "K9u_6E9tO8yr6Jx1D_lHz2tRhSIThPrXpmgazFw8BWI" --output "./asd.txt"***

***For a tiny bit better performance build with release flag:***

***$ cargo run --release -- --transaction "K9u_6E9tO8yr6Jx1D_lHz2tRhSIThPrXpmgazFw8BWI" --output "./asd.txt"***
