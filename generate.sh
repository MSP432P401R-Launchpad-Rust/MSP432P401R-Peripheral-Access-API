svd2rust -i MSP432P401R.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
