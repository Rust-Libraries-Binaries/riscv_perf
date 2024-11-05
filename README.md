# riscv_perf

**riscv_perf** is a performance profiling library for RISC-V programs. It enables RISC-V developers and hobbyists to collect and visualize performance metrics, providing valuable insights into the efficiency of their programs. Whether you're developing software for RISC-V microcontrollers or experimenting with RISC-V emulators, `riscv_perf` offers a practical tool to optimize your code.

## Features
- **Instruction Counting**: Track the number of instructions executed during program execution.
- **Program Loading**: Load RISC-V binary programs into the profiler's memory.
- **Profiling Execution**: Run and profile the loaded programs.
- **Data Visualization**: Visualize profiling data, such as the number of instructions executed.

## Getting Started
### Prerequisites
- Rust programming language installed
- Basic understanding of RISC-V architecture

### Installation
Add `riscv_perf` to your `Cargo.toml`:
```toml
[dependencies]
riscv_perf = "0.1.0"
```
## Usage Example
Here’s a simple example of how to use riscv_perf in your Rust project:

Create main.rs
```rust
use riscv_perf::{profile_program, visualizer};

fn main() {
    // Example RISC-V binary program
    let program = vec![0x00, 0x00, 0x00, 0x33]; // Example RISC-V binary

    // Profile the program
    let profile_data = profile_program(&program);

    // Visualize the profiling data
    visualizer::visualize_profile_data(&profile_data);
}
```
## Running the Tests
To run the unit tests, use the following command:
```sh
cargo test
```
## Contributing
Contributions from anyone interested in improving this library are welcome. Feel free to fork the repository, make your changes, and submit a pull request.

## License
This project is licensed under the MIT License.

## Author
Ben Santora <bensatlantik@gmail.com>

## Donate
https://bensatlantik.github.io/
