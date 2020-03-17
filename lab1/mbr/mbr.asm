[bits 16]
[org 0x7c00]

sti
mov si, BLANK
mov bl, 0
clear:
  lodsb
  cmp al, 0
  je clear_num
  mov ah, 0x0e
  int 0x10
  jmp clear
clear_num:
  inc bl
  cmp bl, 24
  je pre_print
  jmp clear

pre_print:
  mov si, LINE_TEXT
  mov bl, 0
print:
  lodsb
  cmp al, 0
  je print_stop
  mov ah, 0x0e
  int 0x10
  jmp print
print_stop:
  mov al, [0x046c]
wait_timer:
  cmp al, [0x046c]
  je wait_timer
  inc bl
  cmp bl, 18
  je pre_print
  jmp print_stop

.data:
  BLANK times 80 db ' ', 13, 10, 0
  LINE_TEXT db 'Hello World!', 13, 10, 0
  times 510 - ($ - $$) db 0
  dw 0xaa55
