# ActionKV

A simple, persistent key-value database implemented in Rust with a command-line interface.

## Overview

ActionKV is a lightweight key-value store that persists data to disk in a custom binary format. It features an in-memory index for fast lookups and supports basic CRUD operations through a command-line interface.

## Features

- **Persistent Storage**: Data is stored in a custom binary format with CRC32 checksums for data integrity
- **In-Memory Index**: Fast key lookups using an in-memory HashMap index
- **Data Integrity**: CRC32 checksums protect against data corruption
- **Append-Only**: New records are appended to the file, making writes efficient
- **Cross-Platform**: Works on Windows, Linux, and macOS

## Installation

Make sure you have Rust installed, then clone and build the project:

```bash
git clone https://github.com/Stan-breaks/actionkv
cd actionkv
cargo build --release
```

The binary will be available at `target/release/akv_mem` (or `akv_mem.exe` on Windows).

## Usage

### Command Syntax

**Linux/macOS:**
```bash
akv_mem FILE get KEY
akv_mem FILE delete KEY  
akv_mem FILE insert KEY VALUE
akv_mem FILE update KEY VALUE
```

**Windows:**
```cmd
akv_mem.exe FILE get KEY
akv_mem.exe FILE delete KEY
akv_mem.exe FILE insert KEY VALUE  
akv_mem.exe FILE update KEY VALUE
```

### Examples

```bash
# Insert a key-value pair
./akv_mem mydb.db insert "username" "john_doe"

# Retrieve a value
./akv_mem mydb.db get "username"

# Update an existing key
./akv_mem mydb.db update "username" "jane_doe"

# Delete a key (sets value to empty)
./akv_mem mydb.db delete "username"
```

## File Format

ActionKV uses a custom binary format for storage:

```
[4 bytes: CRC32 checksum]
[4 bytes: key length]
[4 bytes: value length]  
[key data]
[value data]
```

Each record is checksummed using CRC32 to detect data corruption.

## Architecture

### Core Components

- **ActionKV**: Main database struct that manages file I/O and indexing
- **KeyValuePair**: Represents a key-value record
- **Index**: In-memory HashMap for fast key lookups, serialized to disk using bincode

### Key Methods

- `open(path)`: Opens or creates a database file
- `load()`: Builds the in-memory index by scanning the entire file
- `get(key)`: Retrieves a value by key using the index
- `insert(key, value)`: Appends a new record to the file
- `update(key, value)`: Alias for insert (updates by appending)
- `delete(key)`: Marks a key as deleted by setting an empty value

### Index Management

The database maintains an in-memory index that maps keys to file positions. This index is periodically serialized and stored in the database file itself under a special key (`+index`). On startup, the database rebuilds the index by scanning all records.

## Dependencies

- `byteorder`: For binary serialization
- `crc32fast`: For data integrity checksums  
- `serde`: For serialization framework
- `bincode`: For efficient binary serialization of the index

## Limitations

- **No Compaction**: Deleted records are not removed from disk, leading to file bloat over time
- **Single-Threaded**: No concurrent access support
- **Memory Usage**: The entire index is kept in memory
- **No Transactions**: Operations are not atomic across multiple keys

## Performance Characteristics

- **Reads**: O(1) average case with in-memory index
- **Writes**: O(1) append-only writes  
- **Startup**: O(n) to rebuild index by scanning file
- **Memory**: O(k) where k is the number of unique keys

## Error Handling

The database will panic if data corruption is detected (CRC32 mismatch). All I/O operations return `Result` types for proper error handling.


