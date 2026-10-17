#If bit 0 in the ‘flags’ word is set, then all boot modules loaded along with the operating system must be aligned on page (4KB) boundaries. Some operating systems expect to be able to map the pages containing boot modules directly into a paged address space during startup, and thus need the boot modules to be page-aligned.
.set MALIGN, 1<<0

#if bit 1 in the ‘flags’ word is set, then information on available memory via at least the ‘mem_*’ fields of the Multiboot information structure (see Boot information format) must be included. If the boot loader is capable of passing a memory map (the ‘mmap_*’ fields) and one exists, then it may be included as well
.set INFO, 1<<1

#The field ‘flags’ specifies features that the OS image requests or requires of an boot loader. Bits 0-15 indicate requirements; 
.set FLAGS, MALIGN | INFO

#The field ‘magic’ is the magic number identifying the header, which must be the hexadecimal value 0x1BADB002. 
.set MAGIC, 0x1BADB002

#The field ‘checksum’ is a 32-bit unsigned value which, when added to the other magic fields (i.e. ‘magic’ and ‘flags’), must have a 32-bit unsigned sum of zero. 
.set CHECKSUM, -(MAGIC+FLAGS)

.section .multiboot
   #The Multiboot header must be contained completely within the first 8192 bytes of the OS image, and must be longword (32-bit) aligned.
.align 4
   .long MAGIC
   .long FLAGS
   .long CHECKSUM

.section .bss
.align 16
stack_bottom:
.skip 16384 #4kb mem
stack_top:

.section .text
.global _start
.type _start, @function
_start:
   mov $stack_top,%esp
   push %ebx #sec arg
   push %eax #fst arg
   call kernel_main
   cli
1: hlt
   jmp 1b
.size _start, . - _start

