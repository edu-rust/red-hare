# RedHare - High-Performance Rust Key-Value Storage Engine

## Introduction

`RedHare` is a high-performance key-value storage engine written in Rust, inspired by Redis. It provides basic key-value storage functionality with value operations and expiration time management, designed for applications requiring high concurrency and memory safety.

## Core Features

- ⚡ **High Performance Concurrency**: Thread-safe concurrent access implemented based on `DashMap`
- 🛡️ **Memory Safety**: Fully leverages Rust's ownership system to avoid memory leaks and data races
- ⏰ **Expiration Management**: Supports setting expiration times for key-value pairs with automatic cleanup
- 🧠 **Lightweight Design**: Focuses on core functionality while maintaining clean and efficient code

## Main Components

### `MetaData`
Internal data structure containing:
- `value`: Stores the actual byte data
- `expire_time`: Optional expiration timestamp

## Use Cases

- Caching systems
- Session storage
- Real-time data processing
- Data sharing between microservices

## Technical Advantages

- **Zero-copy reading**: Optimized data access paths reduce unnecessary memory copying
- **Automatic expiration cleanup**: Lazy deletion mechanism ensures high performance while maintaining data consistency
- **Full UTF-8 support**: Comprehensive string encoding handling
- **Error handling**: Comprehensive error handling mechanism with clear error messages

## Performance Characteristics

- Lock-free concurrent access improves performance in multi-threaded environments
- Memory-friendly data structure design
- Efficient expired key detection mechanism

## Development Status

The project is currently in active development with basic string operation functionality implemented. Future plans include expanding to more data structures and advanced features.