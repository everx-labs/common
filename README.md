# sdk-emulator
Assembler and Node emulator

## Prerequsities

https://www.rust-lang.org/en-US/install.html

## To Build & Run:

```
cargo build
cargo run
```

## To Test:
```
cargo test
```

## Compile smart contract:

After build project you can use **compile** util from `target/release/compile` or `target/debug/compile` for compile your contract

Commands (by unix example):
```
- Compile
  ./compile your_bytecode_file your_cells_file
- Get help
  ./compile --help
```

## Execute smart contract:

After build project you can use **execute** util from `target/release/execute` or `target/debug/execute` for execute your contract

Commands (by unix example):
```
- ./execute your_contract_file
- ./execute --help
```
