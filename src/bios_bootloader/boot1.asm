bits 16         ; for now, still in real mode
org 0x7c00      ; place in the magic boot address

boot:
  mov ax, 0x2401
  int 0x15            ; enable A20 bit
  mov ax, 0x3
  int 0x10            ; enable VGA text mode 3

  lgdt [gdt_pointer]  ; load the gdt table
  mov eax, cr0
  or eax, 0x1         ; set the protected mode bit on special CPU reg cr0
  mov cr0, eax
  jmp CODE_SEG:boot2  ; long jump to code segment

  gdt_start:    ; gdt format, check  wikipedia for details
      dq 0x0
  gdt_code:
      dw 0xFFFF
      dw 0x0
      db 0x0
      db 10011010b
      db 11001111b
      db 0x0
  gdt_data:
      dw 0xFFFF
      dw 0x0
      db 0x0
      db 10010010b
      db 11001111b
      db 0x0
  gdt_end:

  gdt_pointer:    ; gdt pointer structure
      dw gdt_end - gdt_start
      dd gdt_start
  CODE_SEG equ gdt_code - gdt_start ;code and data segments
  DATA_SEG equ gdt_data - gdt_start

bits 32 ; 32 bits baby!
boot2:
	mov ax, DATA_SEG
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax
	mov esi,hello
	mov ebx,0xb8000
.loop:
	lodsb
	or al,al
	jz halt
	or eax,0x0100
	mov word [ebx], ax
	add ebx,2
	jmp .loop
halt:
	cli
	hlt
hello: db "Hello world!",0

times 510 - ($-$$) db 0
dw 0xaa55
