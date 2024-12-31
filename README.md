Tauri + SvelteKit + TypeScript

---

## Backend

> _design? memory? error handling? speed?_
>
> > I designed this backend with a few key concerns in mind:
> > -- **Memory management** was my biggest worry - since VR8 files can be any size, from few MB to 4GBs, just loading them straight into mem could crash the app or user's pc. To handle this, I made the converter check file sizes first. If a file is bigger than _30MB_, it processes it in small 64KB chunks instead of all at once.
> > -- Things can go wrong when dealing with **IO**. files might not exist, be corrupted, or we might not have permission to read. I created custom error types to catch and handle these cases properly, though admittedly the end user might not see the detailed errors (later might add backend error Toasts)
> > -- For _Execution speed_, I used **[Rayon](https://docs.rs/rayon/latest/rayon/)** to process multiple files at the same time. While file operations are usually limited by disk speed, parallel processing can still help, especially when the OS caches things in memory.

#### Implements?

1. **Memory usage and safety**

   - adaptive chunk processing for large files
   - direct mem processing for small files.

2. **Error handling**

   - comprehensive error types
   - wraps IO and WAV errors

3. **Safe Concurrency with Rayon**

   - parallel file processing
   - thread-safe progress tracking
   - optimizes IO throughput
   - atomic counters

---

#### 1. Get Input Files

```rust
// validate paths exist
let path = std::path::Path::new(&path_str);
let parent_dir = path.parent().unwrap_or(std::path::Path::new("."));

// find
let file = std::fs::read_dir(parent_dir)
   .map_err(|e| e.to_string())?
   .filter_map(Result::ok)
   .find(|entry| entry.path() == path)?;
```

#### 2. Output Directory

```rust
// Create unique output dir
let base_path = std::path::Path::new(&output_dir).join("converted-files");
let output_dir = get_unique_dir(base_path)?;
std::fs::create_dir_all(&output_dir)?;
```

#### 3. File Processing

```rust
// Large file handling (>30MB)
if file_size > chunk_threshold {
    let mut reader = std::io::BufReader::with_capacity(64 * 1024, file);
    // Process in 64KB chunks
}

// Smaller file handling
else {
    let mut reader = std::io::BufReader::new(file);
    // Load entire file in memory
}

// Convert chunks to WAV samples
for chunk in buffer[..bytes_read].chunks(2) {
    let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
    writer.write_sample(sample)?;
}
```

#### 4. Progress tracking

```rust
// Track parallel conversion progress
let processed_files = std::sync::atomic::AtomicUsize::new(0);
let progress = ((current + 1) as f64 / total_files as f64 * 100.0) as u32;
window.emit("conversion_progress", progress);
```

#### 5. Results

```rust
// Return conversion stats
Ok(ConversionResult {
    files_converted: files.len(),
    duration_ms: start.elapsed().as_millis(),
    output_dir: output_dir.to_string_lossy().into_owned(),
})
```

#### 6. Error types

```rust
pub enum ConverterError {
    IoError(std::io::Error),  // File ops
    WavError(hound::Error),   // WAV encoding issues
    InvalidData(String),      // Invalid input data
}
```
