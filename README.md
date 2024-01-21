#### ICP with Rust

#### Install dependencies

```bash
# 1. Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# 2. Web Assembly
rustup target add wasm32-unknown-unknown

# 3. Candid Extractor
cargo install candid-extractor

# 4. DFX (DFINIT EXTENSION)
DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"

# 5. NodeJs
```

#### Run

```bash
# 1. Start
dfx start --clean

# 2. Run
npm run gen-deploy
```
