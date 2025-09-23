
# Contents

- [Interaction](#interaction)
  - [From Binary](#from-binary)
  - [With Cargo](#with-cargo)
- [Building](#building)
  - [Instruction Compendium (Expanded Instruction Set)](#instruction-compendium-expanded-instruction-set)
  - [Requirements](#requirements)
- [Testing](#testing)
- [Environment](#environment)
- [Requirements](#requirements-2)
- [Checking Test Cases](#checking-test-cases)

# Interaction

The commandline interface is simple and consists of a single argument `-i/--input` which takes a file (path). An output of disassembled bytes will following. 

## From Binary

```bash
╭─⦗0⦘─⦗user@host:/path/to/disassembler⦘
╰─➤ ./target/release/disassembler -i ./src/tests/files/file1.o
    Finished `release` profile [optimized] target(s) in 0.10s
     Running `target/release/disassembler -i ./src/tests/files/file1.o`
00000000: 31 C0                 xor eax, eax
<... omitted ...>
00000023: C2 08 00              retn 0x0008
```


## With Cargo

```bash
╭─⦗0⦘─⦗user@host:/path/to/disassembler⦘
╰─➤ cargo run --release -- -i ./src/tests/files/file1.o
    Finished `release` profile [optimized] target(s) in 0.10s
     Running `target/release/disassembler -i ./src/tests/files/file1.o`
<...
  Same output from above
...>
```

# Building

```bash
╭─⦗0⦘─⦗user@host:/path/to/disassembler⦘
╰─➤ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.13s
```

## Instruction Compendium (Expanded Instruction Set)

There is a feature that can be enabled to expand the set of instruction supported by the application. They are locked behind the "instruction_compendium" feature flag. This added `imul`, `int3`, and some other instructions that were a part of instructor provided resources but were not listed as part of the **Supported Mnemonics** section. 

The default build will still pass just with the reduces instruction set (computer?). To enable them build with with the following flags:

```bash
╭─⦗0⦘─⦗user@host:/path/to/disassembler⦘
╰─➤ cargo build --features "instruction_compendium"
   Compiling disassembler v0.1.0 (/path/to/disassembler)
    Finished `release` profile [optimized] target(s) in 15.13s
```


## Requirements

Note, requires:
- An _internet connection_
- `cargo`
- `rustc`


# Testing 

The application as multiple tests. These range from input from instructor provided materials, modified or self derived materials, and even a test with just random bytes. 

```bash
╭─⦗0⦘─⦗user@host:/path/to/disassembler⦘
╰─➤ cargo test
warning: unused manifest key: package.author
   Compiling disassembler v0.1.0 (/path/to/disassembler)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.78s
     Running unittests src/main.rs (target/debug/deps/disassembler-3df5991f70f00c15)

running 25 tests
test instruction::encoding::operands::rel32_calculation ... ok
test tests::instruction::compendium::rmi_regrm_and_immediate ... ok
<... omitted ..>
test tests::instruction::edge_cases::random_bytes_as_input ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```

# Environment

- Rust:
  - cargo 1.85.1 (d73d2caf9 2024-12-31)
  - rustc 1.85.1 (4eb161250 2025-03-15)
- System:
  - OS: Ubuntu 22.04.5 LTS x86_64            
  - Kernel: 6.8.0-79-generic                 
  - Shell: bash 5.1.16                       
  - Terminal: /dev/pts/0                     
  - CPU: Intel Celeron N4000 (2) @ 2.600GHz  
  - GPU: Intel GeminiLake [UHD Graphics 600] 

# Requirements
  
- Not crash on any (in)valid inputs.
- Use either the linear sweep or recursive descent algorithm.
- Print disassembled instructions to standard output.
- Handle jumping/calling forwards and backwards, adding labels where appropriate with the following form (see Example 2 below). `offset_XXXXXXXXh:`
- Handle unknown opcodes by printing the address, the byte, and the assembly as follows: `00001000: <byte> db <byte>`
- Work on the supplied examples in addition to other test files that are not supplied.
- Implement only the given opcodes detailed in the Supported Mnemonics section.
- Implements both `SIB` and `MODRM` bytes.
- Negative `disp8` must be handled properly. Example (either display format is acceptable): `[ esi - 0x4 ]` or `[ esi + 0xfffffffc ]`
- Have the input file specified using the “-i" command-line option. Example: `./disassembler -i example1`
- Display only addresses, instruction machine code (i.e. the bytes that make up the instruction), disassembled instructions/data, and labels.

Example:
```
00000000 50 push eax
00000001 E802000000 call offset_00000008h
00000006 90 nop
00000007 C3 ret
offset_00000008h:
00000008 C3 ret
```

# Checking Test Cases

One use `nasm` or `as` to directly pass bytes to the assembler and then use a disassembler to view the resulting instruction. For `nasm` we would use a file like:
```asm
[BITS 32]
db 0x75
db 0x0F
```
And confirm output via `nasm -f elfx32 assembly.s -o assembly.o && objdump -d -M intel assembly.o`

