# Rust basics by Bek Brace

YouTube [playlist](https://www.youtube.com/playlist?list=PLrOQsSoS-V69UWKxV4FNRJFlHS0DUFQA2)

## Primitive (scalar) Data Types

YouTube [playlist](https://www.youtube.com/watch?v=ETGNzcmfznY&list=PLrOQsSoS-V69UWKxV4FNRJFlHS0DUFQA2&index=2)

### Integers

- singed: i8, i16, i32, i64, i128
- unsigned: u8, u16, u32, u64, u128

### Floats

- f32
- f64

### Booleans

- true
- false

### Chars

- char

## Compound Data Types

YouTube [playlist](https://www.youtube.com/watch?v=0IeGjnQfnb0&list=PLrOQsSoS-V69UWKxV4FNRJFlHS0DUFQA2&index=2)

### Arrays

```rust
let numbers:[i32; 5] = [1, 2, 3, 4, 5];
```

### Tuples

```rust
let human:(String, i32, bool) = ("Mario", 49, true);
```

### Slices

```rust
let num_slices: &[i32] = &[1, 2, 3, 4, 5];
```

#### Strings vs String slices
- Strings: growable, mutable, owned string type
- String slice: immutable reference

```rust
 // String is mutable
let mut name: String = String::from("Mario");
name.push_str(" Lazzari");
println!("My name is {}", name);

// String slice
let string: String = String::from("My string");
let slice: &str = &string;
println!("Slice value is {}", slice);
let slice_str: &str = &string[0..3];
println!("First 3 chars: {}", slice_str);
```




### Strings

#### Slice strings