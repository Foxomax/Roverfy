# Roverfy

[![Rust CI](https://github.com/Foxomax/Roverfy/actions/workflows/ci.yml/badge.svg)](https://github.com/Foxomax/Roverfy/actions/workflows/ci.yml)

Roverfy is a web development framework created in Rust, inspired by Django. Its goal is to provide a complete and easy-to-use solution for building robust and scalable web applications in Rust, with a focus on performance, type-safety, and a great developer experience.

## Table of Contents
- [Features](#features)
- [Getting Started](#getting-started)
- [Contributing](#contributing)
- [License](#license)

## Features

Roverfy is currently under active development. Here are some of the implemented features and what's planned for the future.

### Current Features
- **HTTP Server**: A basic, multi-threaded TCP server to handle incoming connections.
- **HTTP Request & Response Parsing**: Robust parsing for HTTP requests, including headers, methods, and body content (JSON and Form-encoded).
- **Type-Safe HTTP Abstractions**: Clear, type-safe structs for `Request`, `Response`, `StatusCode`, `Method`, and `ContentType`.
- **Server-Side Rendering (SSR)**: Built-in integration with the [Tera](https://keats.github.io/tera/) template engine for powerful and flexible server-side rendering.
- **Flexible Configuration**: A trait-based configuration system that allows users to provide their own settings, with a smart default that automatically finds the project root.
- **Continuous Integration**: A ready-to-use GitHub Actions workflow that automatically checks, tests, and validates every change.

### Roadmap
- Advanced Routing System
- Middleware Support
- Static File Serving
- Database Integration (ORM-like features)
- Session and Authentication Management

## Getting Started

Follow these steps to get a local copy of Roverfy up and running.

### Prerequisites
You need to have the Rust toolchain installed on your machine. If you don't have it yet, you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation & Usage

At the moment, Roverfy is not yet published on `crates.io`. To use it, you can clone the repository and run the example project:

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/Foxomax/Roverfy.git
    cd Roverfy
    ```

2.  **Run the example application:**
    This will start the development server on `127.0.0.1:7878`.
    ```bash
    cargo run --example main
    ```

### Example Code

The core of a Roverfy application is simple to set up. Here is the code from the example (`examples/main.rs`):

```rust
use std::env;
use roverfy::{BaseSettings, Roverfy};

fn main() {
    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Initialize the default settings.
    // BaseSettings automatically finds the project root to locate templates.
    let settings = BaseSettings::default();

    // Create a new Roverfy app instance.
    let app = Roverfy::new(args, settings);

    // Run the application.
    // This will handle parsing commands like "serve" and starting the server.
    if let Err(e) = app.run() {
        eprintln!("Error: {}", e);
    }
}
```

## Contributing
We'd love for you to contribute to Roverfy! If you have ideas for new features, improvements, or bug fixes, please follow these steps:

1.  Fork the repository.
2.  Create a new branch: `git checkout -b new-feature`
3.  Make your changes and commit them: `git commit -m 'Add new feature'`
4.  Push your branch: `git push origin new-feature`
5.  Create a pull request with a description of your changes.

Please follow the coding standards and ensure all tests pass before submitting a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
