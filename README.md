# Markdown Server
A simple custom webserver that consumes markdown files and renders HTML. This project only uses Rust's standard libarary.

To build and run the server:
- Place markdown files in `src/pub`. The server will serve index.md by default.
- Run `cargo run` in the repository root.
- Access `http://localhost:8080` in a web browser.

# Supported syntax
The server currently supports:

- Headers
- Subheaders
- Sub-Subheaders
- Code blocks
- Unordered lists
- Images
