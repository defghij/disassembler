
# Deliverables 

1. Brief discussion (about 1 page) on the strengths and weaknesses of the recursive descent and linear sweep algorithms. What makes tools like IDA and Ghidra powerful disassemblers?
2. Your source code for a disassembler for a small subset of the Intel Instruction Set, as described in the remainder of this assignment.

# Requirements
Be written in any of the following programming languages: C, C++, Go, Rust, Java,
Python. Please ask the instructor if you have issues with this requirement.
- Not crash on any (in)valid inputs.
- Use either the linear sweep or recursive descent algorithm. Most students choose linear sweep.
- Print disassembled instructions to standard output.
- Handle jumping/calling forwards and backwards, adding labels where appropriate with the following form (see Example 2 below). `offset_XXXXXXXXh:`
- Handle unknown opcodes by printing the address, the byte, and the assembly as follows
(see skeleton code for an example). `00001000: <byte> db <byte>`
- Work on the supplied examples in addition to other test files that are not supplied.
- Implement only the given opcodes detailed in the Supported Mnemonics section.
- Implements both `SIB` and `MODRM` bytes.
- Negative `disp8` must be handled properly.
Example (either display format is acceptable):
00000000: 017EFC add [ esi - 0x4 ], edi
00000000: 017EFC add [ esi + 0xfffffffc ], edi
- Have the input file specified using the “-i" command-line option. Example: `./disassembler -i example1`
- Display only addresses, instruction machine code (i.e. the bytes that make up the instruc-
tion), disassembled instructions/data, and labels.
Example:
```
00000000 50 push eax
00000001 E802000000 call offset_00000008h
00000006 90 nop
00000007 C3 ret
offset_00000008h:
00000008 C3 ret
```

