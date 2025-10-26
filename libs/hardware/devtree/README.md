# DevTree - Zero-Copy Device Tree Library

A zero-copy device tree parser for embedded Rust systems.

## Features

- Zero-copy design - no memory allocation
- Iterator-based API for traversing nodes and properties
- Support for finding nodes and properties by name
- Memory reservation map access
- Compatible with `#![no_std]` environments

## Usage

```rust
use devtree::{DevTree, DevTreeNode};

// Get DTB pointer from bootloader (typically in x0 register)
let dtb = DevTree::new(dtb_ptr)?;

// Access root node
let root = dtb.root();

// Iterate over properties
for prop in root.properties() {
    println!("Property: {} = {:?}", prop.name(), prop.value());
}

// Find specific property
if let Some(prop) = root.property("compatible") {
    if let Some(compat_str) = prop.as_string() {
        println!("Compatible: {}", compat_str);
    }
}

// Iterate over child nodes
for child in root.children() {
    println!("Child node: {}", child.name());
}

// Find specific child node
if let Some(cpu_node) = root.child("cpus") {
    // Work with CPU node...
}
```

## API

### DevTree
- `new(dtb_ptr: *const u64) -> Option<Self>` - Create from DTB pointer
- `root() -> &DevTreeNode` - Get root node
- `header() -> &DtbHeader` - Get DTB header
- `mem_rsvmap() -> &[u64]` - Get memory reservation map

### DevTreeNode
- `name() -> &str` - Get node name
- `properties() -> PropertyIterator` - Iterate over properties
- `children() -> ChildNodeIterator` - Iterate over child nodes
- `property(name: &str) -> Option<Property>` - Find property by name
- `child(name: &str) -> Option<DevTreeNode>` - Find child by name

### Property
- `name() -> &str` - Get property name
- `value() -> &[u8]` - Get raw property value
- `as_string() -> Option<&str>` - Get as null-terminated string
- `as_u32() -> Option<u32>` - Get as 32-bit integer (big-endian)
- `as_u64() -> Option<u64>` - Get as 64-bit integer (big-endian)
