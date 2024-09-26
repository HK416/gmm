# Game Math for Me
A math library for video games that allow independent management of data.
Just like [DirectXMath](https://github.com/microsoft/DirectXMath), data and vectors are divided.

# Design
### Data Type
Data types are designed to be used when storing data in files or exchanging it with other systems.

#### Example
```rust
use gmm::Float4;
 
let a = Float4::new(1.0, 2.0, 3.0, 4.0);
let s = Float4::fill(5.0);
let res = a + s;
 
println!("{} + {} = {}", a, s, res);
```

### Vector Type
Vector types are designed to be used when processing calculations.
Some systems use `SIMD` instructions.

#### Example
```rust
use gmm::Vector;
 
let a = Vector::new(1.0, 2.0, 3.0, 4.0);
let s = Vector::fill(5.0);
 
let res = a + s;
 
println!("{} + {} = {}", a, s, res);
```


# Features
### Supports SIMD operations
The gmm library supports SIMD instructions on the `x86`, `x86_64` and the `aarch64` architecture.

### Compile Features
- `bytemuck` - Enables the bytemuck library implementation.
- `serde` - Enables the serde library implementation.
- `mint` - Enables the mint library implementation.
- `scalar-math` - Disable the simd instruction in the library.
- `use-assertion` - Allow panic calls within a function.

# License
MIT license (LICENSE or http://opensource.org/licenses/MIT)
