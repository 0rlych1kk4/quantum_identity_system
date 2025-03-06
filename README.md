# README.md
# Quantum-Protected Digital Identity System

## Overview
A Rust-based quantum-resistant digital identity system using post-quantum cryptography (Dilithium and NTRU). This system provides secure identity generation and verification using a command-line tool and a WebAssembly module.

## Features
- **Quantum-Resistant Cryptography**: Uses **Dilithium** for digital signatures and **NTRU** for encryption.
- **Command-Line Interface (CLI)**: Generate and verify quantum-protected identities.
- **WebAssembly (WASM) Support**: Enables integration with web applications.

## Installation
Ensure you have Rust installed:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository and navigate to the project:
```sh
git clone https://github.com/your-repo/quantum_identity_system.git
cd quantum_identity_system
```

## Building and Running
### Build the project
```sh
cargo build --release
```

### Generate a Quantum Identity
```sh
cargo run --release --bin identity_cli -- --generate
```

### Verify a Quantum Identity
```sh
cargo run --release --bin identity_cli -- --verify identity.json
```

## WebAssembly (WASM) Usage
Compile the WASM module:

cd identity_wasm
wasm-pack build --target web
```

Use in a JavaScript project:
```js
import init, { generate_identity, verify_identity } from "./pkg/identity_wasm.js";
async function main() {
    await init();
    let identity = generate_identity();
    console.log("Generated Identity:", identity);
    console.log("Verification:", verify_identity(identity));
}
main();
```

## License
MIT

