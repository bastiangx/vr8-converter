<div align="center">
<h1>🦀 Technical Details</h1>
</div>

This project is split into three main components:

- `vr8-core`: The core conversion library
- `vr8-cli`: Command-line interface tool
- `vr8-gui`: Tauri + SvelteKit desktop app

## 🔧 Core Design Principles

### Memory Management

```rust
// Smart chunk-based processing for large files (>30MB)
if file_size > chunk_threshold {
    let mut reader = std::io::BufReader::with_capacity(64 * 1024, file);
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        // Process in 64KB chunks
    }
} else {
    // Direct memory processing for small files
    let mut reader = std::io::BufReader::new(file);
    reader.read_to_end(&mut data)?;
}
```

### Parallel Processing

- Uses Rayon for parallel file conversion
- Thread-safe progress tracking using atomic counters
- Progress reporting for both CLI and GUI:

```rust
let pb = ProgressBar::new(total_files); // CLI progress
if let Some(w) = &window {
    w.emit("conversion_progress", progress)?; // GUI progress
}
```

### Error Handling

Comprehensive error types for both CLI and GUI:

```rust
pub enum ConverterError {
    IoError(std::io::Error),    // File system operations
    WavError(hound::Error),     // WAV encoding
    InvalidData(String),        // Bad input data
}
```

### CLI Features

- File path validation
- Progress bar
- Human-readable duration formatting:

```rust
fn format_duration(ms: u128) -> String {
    if ms < 1000 { format!("{}ms", ms) }
    else if ms < 60000 { format!("{:.1}s", ms as f64 / 1000.0) }
    else {
        format!("{}m {}s", ms / 60000, (ms % 60000) / 1000)
    }
}
```

### Shared Core Logic

Both CLI and GUI use the same core converter:

```rust
pub async fn convert_files(
    window: Option<Window>,     // Optional GUI window
    paths: Vec<String>,         // Input files
    output_dir: String,         // Output directory
) -> Result<ConversionResult, String>
```
