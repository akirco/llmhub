# LLMHub - Rust Client for Large Language Models

A Rust-powered client library for interacting with large language models (LLMs). Supports streaming and non-streaming conversations with various API providers.

## Notice

This is a work in progress and is not yet ready for production use.some providers are maybe supported yet.

## Models supported

`checkout /src/models/models.rs for more details.`

## Providers supported

`checkout /src/api/providers.rs for more details.`


## Features ✨

- 🚀 Async-first implementation using Tokio runtime
- 🌐 Multi-provider support (Deepseek, etc.)
- 📡 Stream response handling with backpressure support
- 🔧 Configurable API endpoints and rate limiting
- 🛠️ Strong type safety with Rust enums and structs
- 🧠 Conversation memory management
- 🚦 Comprehensive error handling

## Installation 📦

Add to your `Cargo.toml`:

```toml
[dependencies]
llmhub = { git = "https://github.com/akirco/llmhub" }
```

## Usage 🚀

`checkout examples`

```rust
cargo run --example llmhub_test
```

## Contributing 🤝

Feel free to open issues or pull requests.

## License 📄

MIT License