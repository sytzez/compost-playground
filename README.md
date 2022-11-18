# The Compost Playground

An online playground for the [Compost Programming Language](https://github.com/sytzez/compost).

## Quick Start

To run the server locally, clone the compost repository in the same parent folder as this repository and run:

```bash
cargo run
```

The playground will then be available on http://localhost:8080.

## Architecture

The backend runs on Rust using the [Actix](https://actix.rs/) framework.
The frontend uses [Bootstrap 5](https://getbootstrap.com/) and vanilla JavaScript.
It also works with JavaScript disabled, though slightly less user friendly.
