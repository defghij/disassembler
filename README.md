

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

