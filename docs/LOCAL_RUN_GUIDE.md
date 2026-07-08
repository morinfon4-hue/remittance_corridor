# Local Run Guide

## Check repository

<pre>
git remote -v
git status
</pre>

## Run contract tests

<pre>
cargo test --workspace
</pre>

## Build contract

<pre>
cargo build --workspace --target wasm32v1-none --release
</pre>

## Run frontend

<pre>
cd frontend
npm install
npm run dev
</pre>
