# Quality and Verification

## Contract Test

<pre>
cargo test --workspace
</pre>

## Contract Build

<pre>
cargo build --workspace --target wasm32v1-none --release
</pre>

## Frontend Test

<pre>
cd frontend
npm test
</pre>

## Frontend Build

<pre>
cd frontend
npm run build
</pre>

## Generated Files Policy

Do not commit generated files:

<pre>
target/
node_modules/
dist/
.vite/
contracts/**/test_snapshots/
*.tsbuildinfo
frontend/vite.config.js
frontend/vite.config.d.ts
</pre>
