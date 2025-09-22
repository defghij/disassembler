[BITS 32]

; nasm file2.s -o file2.o
; ndisasm -u file2.o > file2.out

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

xor     eax, eax
add     eax, ecx
add     eax, edx
push    ebp
mov     ebp, esp
push    edx
push    ecx
mov     eax, 041424344h
mov     edx, dword [ dword ebp + 08h]   ; The first dword refers to the 
                                        ; memory access, the second refers 
                                        ; to the size of the 
                                        ; immediate (0x00000008).
mov     ecx, dword [ dword ebp + 0ch]   ; The first dword refers to the 
                                        ; memory access, the second refers 
                                        ; to the size of the 
                                        ; immediate (0x0000000c).
add     ecx, edx
mov     eax, ecx
pop     edx
pop     ecx
pop     ebp
retn    08h

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

mov dword [ byte esi - 4 ], edi   ; expected output in disassembler
                                  ; 00000000:  89 7E FC            mov [esi-0x4],edi
                                  ;  -OR-
                                  ; 00000000:  89 7E FC            mov [esi + 0xfffffffc],edi



;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

push    ebp
mov     ebp, esp
push    edx
push    ecx
cmp     ecx, edx
jz      label_error
mov     eax, 041424344h
mov     edx, dword [ byte ebp + 08h]    ; By default, the assembler will 
                                        ; likely make 0x08 a byte, but the
                                        ; byte qualifier guarantees it.
mov     ecx, dword [ byte ebp + 0ch]    ; By default, the assembler will 
                                        ; likely make 0x0c a byte, but the
                                        ; byte qualifier guarantees it.
add     ecx, edx
mov     eax, ecx

label_error:
pop     edx
pop     ecx
pop     ebp
retn    08h


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

push ebp
push edi
retn

my_label:
mov [eax], edi
push ebp
push edi
push ebp
jmp my_label

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;


;clflush esi     ; expected disassmebler ouput
db 0x0F                ; 00000000:  0F                db 0x0f
db 0xAE                ; 00000001:  AE                db 0xae
db 0xFE                ; 00000002:  FE                db 0xfe

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

db 0x8d             ; lea edi, ecx (invalid so need to emit)
db 0xf9
                    ; expected output of disassembler:
                    ;00000000:  8d   db 0x8d
                    ;00000001:  f9   db 0xf9
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; call with missing bytes
db 0xe8             ; expected output from disassembler:
db 0x00
db 0x00

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
