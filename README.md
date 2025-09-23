# Interaction

The commandline interface is simple and consists of a single argument `-i/--input` which takes a file (path). An output of disassembled bytes will following. 

## From Binary

╭─⦗0⦘─⦗user@host:/path/to/disassembler⦘
╰─➤ ./target/release/disassembler -i ./src/tests/files/file1.o
warning: unused manifest key: package.author
    Finished `release` profile [optimized] target(s) in 0.10s
     Running `target/release/disassembler -i ./src/tests/files/file1.o`
00000000: 31 C0                 xor eax, eax
00000002: 01 C8                 add eax, ecx
00000004: 01 D0                 add eax, edx
00000006: 55                    push ebp
00000007: 89 E5                 mov ebp, esp
00000009: 52                    push edx
0000000A: 51                    push ecx
0000000B: B8 44 43 42 41        mov eax, 0x41424344
00000010: 8B 95 08 00 00 00     mov edx, [ ebp + 0x00000008 ]
00000016: 8B 8D 0C 00 00 00     mov ecx, [ ebp + 0x0000000C ]
0000001C: 01 D1                 add ecx, edx
0000001E: 89 C8                 mov eax, ecx
00000020: 5A                    pop edx
00000021: 59                    pop ecx
00000022: 5D                    pop ebp
00000023: C2 08 00              retn 0x0008


## From Source Directory

```bash
╭─⦗0⦘─⦗user@host:/path/to/disassembler⦘
╰─➤ cargo run --release -- -i ./src/tests/files/file1.o
warning: unused manifest key: package.author
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
warning: unused manifest key: package.author
    Finished `release` profile [optimized] target(s) in 0.13s
```

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
test instruction::encoding::operands::rel8_calculation ... ok
test opcodes::access_internal_state ... ok
test opcodes::retrieve_multiple_decode_rules ... ok
test opcodes::retrieve_nonexistent_rule ... ok
test opcodes::retrieve_single_decode_rule ... ok
test output::multiple_line ... ok
test output::single_line ... ok
test output::unknown_byte ... ok
test output::with_label ... ok
test tests::instruction::compendium::immediate ... ok
test tests::instruction::compendium::displacement ... ok
test tests::instruction::compendium::m1_rm_and_one ... ok
test tests::instruction::compendium::m_rm ... ok
test tests::instruction::compendium::mi_rm_and_immediate ... ok
test tests::instruction::compendium::mr_rm_and_reg ... ok
test tests::instruction::compendium::misc_instructions ... ok
test tests::instruction::compendium::opcode_and_immediate ... ok
test tests::instruction::compendium::opcode ... ok
test tests::instruction::compendium::rmi_regrm_and_immediate ... ok
test tests::instruction::compendium::rm_reg_and_rm ... ok
test tests::instruction::compendium::zero ... ok
test tests::instruction::files::file1 ... ok
test tests::instruction::files::file2 ... ok
test tests::instruction::edge_cases::random_bytes_as_input ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```


# Requirements
# 
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

