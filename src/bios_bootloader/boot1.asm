bits 16     ; Work in 16 bits because thats what the cpu will be in
org 0x7c00  ; Place program at this offset (bios expects it here)

boot:
  mov si, hello     ; si register points at hello label
  mov ah, 0x0e      ; 0x0e in ah means write char in tty mode

.loop:
  lodsb             ; load ds:si byte into al
  or al,al          ; is al == 0
  jz halt           ; if true, jump to halt
  int 0x10          ; BIOS interrupt 0x10 (video services)
  jmp .loop         ; restart loop

halt:
    cli             ; clear interupt flag
    hlt             ; halt execution

hello: db "Hello World",0

times 510 - ($-$$) db 0   ; fill the rest 510 bytes with zeroes
dw 0xaa55                 ; magic value that marks this 512 byte sector as boot
