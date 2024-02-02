# light-evm • [![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![CI](https://github.com/lazyfuhrer/light-evm/actions/workflows/tests.yml/badge.svg)](https://github.com/lazyfuhrer/light-evm/actions/workflows/tests.yml) ![Built Using Rust](https://img.shields.io/badge/Built%20Using-Rust-orange.svg) 

`light-evm` is a simple implementation of the EVM (Ethereum Virtual Machine). It's purely an experimental project for my own educational purposes. So don't use any code from this repo for production.

## Getting started:

### Prerequisites

- [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)  
 
### Build Instructions

1. Clone the repository:
   ```bash
   git clone https://github.com/lazyfuhrer/light-evm.git
   cd light-evm
   ```
2. Build the project:
   ```bash
   cargo build
   ```
3. Run the project:
   ```bash
   cargo run <BYTECODE>
   ```
 
 ### Example:
 Run: `cargo run 600660070260005360016000f3`
 
 It should return`0x2a` as the output.

```bash

"PUSH1" @ pc=0
Stack: [6]
Memory: []
---------
"PUSH1" @ pc=2
Stack: [6, 7]
Memory: []
---------
"MUL" @ pc=4
Stack: [42]
Memory: []
---------
"PUSH1" @ pc=5
Stack: [42, 0]
Memory: []
---------
"MSTORE8" @ pc=7
Stack: []
Memory: [42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
---------
"PUSH1" @ pc=8
Stack: [1]
Memory: [42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
---------
"PUSH1" @ pc=10
Stack: [1, 0]
Memory: [42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
---------
"RETURN" @ pc=12
Stack: []
Memory: [42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
---------
Output : 0x2a00000000000000

```
