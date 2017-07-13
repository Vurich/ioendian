# IOEndian

A tiny library to declaratively define endianness in a portable way, for IO.
Allows an easy `.native()` method to convert to native endianness.

## Examples

```rust
struct SomeStruct {
    first: Big<u16>,
    second: Big<i32>,
}

let foo = SomeStruct {
    first: Big([2, 1]),
    second: Big([2, 1, 1, 1]),
};

assert_eq!(foo.first.native() + foo.second.native(), 513 + 33_620_225)
```
