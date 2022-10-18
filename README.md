# rust-downloadr
download arweave tx-data in rust


Run:

***$ cargo run --transaction <tx_id> --output <file_name>***


Example:

***$ cargo run -- --transaction "K9u_6E9tO8yr6Jx1D_lHz2tRhSIThPrXpmgazFw8BWI" --output "./asd.txt"***

***For a tiny bit better performance build with release flag:***

***$ cargo run --release -- --transaction "K9u_6E9tO8yr6Jx1D_lHz2tRhSIThPrXpmgazFw8BWI" --output "./asd.txt"***
