[BITS 32]

int3
cdq
retn
retf
dec ebx
inc eax
dec eax
push ecx
pop edi
push 0xAABBCCDD
int 0x03
add eax, 0xAABBCCDD
offset_one:
jz offset_one
jnz offset_two
jz offset_three
offset_two:
call offset_three
call offset_one
mov ebx, 0xAABBCCDD
push ecx
push dword [ 0xAABBCCDD ]
dec dword [ ebx ]
inc dword [ eax + 0x00000030 ]
push dword [ esi + 0xAABBCCDD ]
inc dword [ eax + ebx + 0xAABBCCDD ]
push dword [ esi + 0xAABBCCDD ]
offset_three:
call [ esp ]
sar dword [ ebx + 0x00000010 ], 0x01
sal dword [ edx * 2 + esi ], 0x01
shr dword [ edi * 2 + 0xAABBCCDD ], 0x01
or edi, 0xAABBCCDD
xor dword [ ebp ], 0xAABBCCDD
cmp esi, edi
xor [ 0xAABBCCDD ], esi
add [ edi + 0xAABBCCDD ], ecx
mov [ esi * 4 + ebx + 0xAABBCCDD ], edi
cmp [ ebx * 4 + 0xAABBCCDD ], eax
cmp edi, esi
add ecx, [ edi + 0xAABBCCDD ]
mov edi, [ esi * 4 + ebx + 0xAABBCCDD ]
cmp eax, [ ebx * 4 + 0xAABBCCDD ]
imul eax, edx, 0x10
imul edi, [ esi * 8 + eax + 0xAABBCCDD ], 0x11223344
imul ebx, [ eax * 8 + 0xAABBCCDD ], 0x11223344
