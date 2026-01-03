# Visual Page Editor (Rust)

A modern, high-performance visual editor for Page XML files, rewritten in Rust.

## Features

- **Fast and Efficient**: Written in Rust for optimal performance
- **Modern GUI**: Built with egui for a responsive, native-feeling interface
- **Full Page XML Support**: Edit all Page XML elements (text regions, images, tables, etc.)
- **Cross-Platform**: Runs on Linux, macOS, and Windows
- **Memory Safe**: Rust's memory safety guarantees prevent common bugs

## Building

### Prerequisites

Install Rust using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/visual-page-editor
```

## Development

### Run in Debug Mode

```bash
cargo run
```

### Run Tests

```bash
cargo test
```

### Check Code

```bash
cargo clippy
cargo fmt
```

## Project Structure

```
src/
├── main.rs          # Application entry point
├── app.rs           # Main application state and logic
├── page_xml.rs      # Page XML parsing and manipulation
├── canvas.rs        # Canvas rendering and interaction
└── ui.rs            # UI components and rendering
```

## Dependencies

- **eframe/egui**: Modern immediate-mode GUI framework
- **quick-xml**: Fast XML parsing
- **roxmltree**: XML tree manipulation
- **image**: Image loading and processing
- **resvg/usvg**: SVG rendering
- **serde**: Serialization framework

## Status

🚧 **Work in Progress**

This is a rewrite of the original JavaScript-based visual-page-editor in Rust. Current status:

- [x] Basic project structure
- [x] GUI framework setup
- [x] Page XML data structures
- [ ] XML parsing implementation
- [ ] Canvas rendering
- [ ] File I/O
- [ ] Edit operations
- [ ] Keyboard shortcuts
- [ ] XSLT support
- [ ] Format import/export

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

MIT License - see LICENSE file for details.

## Acknowledgments

Based on the original [visual-page-editor](https://github.com/buzzcauldron/visual-page-editor) and [nw-page-editor](https://github.com/mauvilsa/nw-page-editor) by Mauricio Villegas.

