# Larian Studios File Format Converter

A Rust-based tool for bidirectional conversion between Larian Studios' LSF (binary) and LSX (XML) file formats, used in games like Baldur's Gate 3 and Divinity: Original Sin.

## Overview

This converter implements robust parsing and conversion between:
- **LSF (Larian Studios Format)**: Compact binary format optimized for game runtime
- **LSX (Larian XML Format)**: Human-readable XML equivalent for modding and debugging

The tool supports LSF versions 6 and 7, including the complex string hash table system and comprehensive attribute type handling.

## Installation

### Prerequisites
- Rust 1.70+ (latest stable recommended)
- Cargo (comes with Rust)

### Building from Source
```bash
git clone <repository-url>
cd packunpack
cargo build --release
```

The compiled binary will be available at `target/release/larian-convert`.

## Usage

### Convert LSF to LSX (Binary to XML)
```bash
# Using cargo run
cargo run -- to-xml input.lsf output.lsx

# Using compiled binary
./target/release/larian-convert to-xml input.lsf output.lsx
```

### Convert LSX to LSF (XML to Binary)
```bash
# Using cargo run
cargo run -- to-binary input.lsx output.lsf

# Using compiled binary  
./target/release/larian-convert to-binary input.lsx output.lsf
```

## Features

### LSF Format Support
- **LSF Version 6 & 7**: Full support for modern Baldur's Gate 3 and Divinity formats
- **String Hash Tables**: Proper parsing of compact hash table string storage (not sequential)
- **5-Chunk Architecture**: Strings, Keys, Nodes, Attributes, and Values chunks
- **Compression Support**: LZ4, Zlib, and Zstd decompression
- **34 Attribute Types**: Complete type system including primitives, vectors, matrices, UUIDs, and complex types

### Robust Parsing
- **Enhanced Error Handling**: Graceful handling of malformed data without crashes
- **Bounds Checking**: Comprehensive validation prevents buffer overflows
- **String Preservation**: High-fidelity string extraction from compact hash tables
- **Type-Safe Parsing**: Rust's type system ensures memory safety

### Data Integrity
- **Round-trip Conversion**: LSF → LSX → LSF maintains data integrity
- **String Hash Resolution**: Sophisticated bucket/chain collision handling
- **Version-aware Parsing**: Handles different LSF versions with appropriate structures

## Architecture

### Core Data Flow
```
LSF File → LSFReader → Resource Object → LSXWriter → LSX File
LSX File → LSXReader → Resource Object → LSFWriter → LSF File
```

### Key Components

**Resource Object Model**
- `Resource`: Root container with metadata and regions
- `Region`: Named containers for node hierarchies  
- `Node`: Tree nodes with attributes and children
- `NodeAttribute`: Typed attribute values supporting 34 different types

**LSF Binary Format Parser**
- Complex string hash table resolution (32-bit packed indices)
- Version-specific metadata handling (LSFMetadataV5/V6)
- Comprehensive attribute type parsing following LSLib patterns
- Enhanced error recovery for malformed files

**LSX XML Format Handler**
- XML serialization/deserialization using quick-xml
- Type-aware attribute value conversion
- Hierarchical structure preservation

## Algorithm Details

### LSF String Storage System

The LSF format uses a sophisticated hash table system for string storage, not sequential arrays:

- **Hash Table Structure**: Fixed number of buckets (typically 512)
- **Collision Handling**: Chain-based collision resolution within buckets
- **32-bit References**: Upper 16 bits = bucket index, lower 16 bits = chain index
- **Compact Format**: Special handling for bucket_count=0 with pattern `[1, 0, length, 0]`

### Attribute Parsing Strategy

Following LSLib's type-driven approach:
- **Match-based Parsing**: Large match statement on AttributeType enum
- **Bounds Validation**: Safety checks for all attribute lengths and offsets  
- **Version Handling**: Different offset calculation for LSF v2 vs v3+
- **Error Recovery**: Individual attribute failures don't stop parsing

### Conversion Process

**LSF to LSX:**
1. Parse binary LSF headers and metadata
2. Decompress 5 data chunks (Strings, Keys, Nodes, Attributes, Values)
3. Resolve string hash table to build lookup system
4. Reconstruct node hierarchy from parent indices
5. Parse typed attributes using enhanced error handling
6. Generate XML with proper structure and formatting

**LSX to LSF:**
1. Parse XML structure into Resource object tree
2. Build string hash table from all unique strings
3. Flatten hierarchy into indexed node/attribute arrays
4. Serialize typed attribute values to binary
5. Compress data chunks and write LSF headers

## Testing

The project includes comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run tests with debug output (recommended for LSF parsing diagnostics)
cargo test -- --nocapture

# Run specific diagnostic test
cargo test test_diagnose_profile8_parsing -- --nocapture
```

### Test Coverage
- **Round-trip Conversion**: Validates LSF → LSX → LSF preserves data
- **String Preservation**: Ensures extracted strings match expectations  
- **Multi-file Testing**: Tests against various LSF file structures
- **Error Handling**: Validates graceful handling of malformed data

## Implementation Notes

### Format Documentation

The `notes/` directory contains detailed algorithmic documentation:

- **`summary.md`**: Complete LSF ↔ LSX conversion algorithm breakdown
- **`metadata.md`**: Deep dive into LSF v7 format and string storage system

These documents provide comprehensive technical details about:
- LSF binary structure and chunk organization
- String hash table algorithms and collision handling  
- Node hierarchy reconstruction from flat arrays
- Attribute type parsing and serialization strategies
- Version differences and compatibility considerations

### Known Limitations

**Attribute Offset Calculation**: Some complex LSF v7 files may have attribute offsets that exceed the values stream bounds. The robust error handling prevents crashes, but some attribute data may not be parsed. This appears to be a limitation in offset interpretation rather than the core parsing logic.

**Performance**: The parser prioritizes correctness and error handling over raw performance. For very large files, consider the trade-offs between safety and speed.

## Contributing

When working on this codebase:

1. **Use debug output**: Always run tests with `-- --nocapture` to see parsing diagnostics
2. **Test thoroughly**: The LSF format is complex - validate changes against multiple test files
3. **Handle errors gracefully**: Maintain the robust error handling approach
4. **Document format details**: The binary format has many subtleties - document discoveries

## License

[Add your license information here]