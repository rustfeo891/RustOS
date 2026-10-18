#![no_std]
#![no_main]

//#[path="io.rs"]
mod io;
mod serial;

mod test;

use core::ops::Add;
use core::panic::PanicInfo;
use core::arch::asm;
//use core::ptr::write_volatile;


#[panic_handler]
fn panic(_panic:&PanicInfo)->!{
   loop{}
}


#[repr(C)]
pub enum VgaColor{
   VgaColorBlack=0,
   VgaColorBlue=1,
   VgaColorGreen=2,
   VgaColorCyan=3,
   VgaColorRed=4,
   VgaColorMagenta=5,
   VgaColorBrown=6,
   VgaColorLightGrey=7,
   VgaColorDarkGrey=8,
   VgaColorLightyBlue=9,
   VgaColorLightGreen=10,
   VgaColorLightCyan=11,
   VgaColorLightRed=12,
   VgaColorLightMagenta=13,
   VgaColorLightBrown=14,
   VgaColorWhite=15
}

pub extern "C" fn vga_entry_color(fg:VgaColor,bg:VgaColor)->u8{
   (fg as u8)|(bg as u8)<<4
}

pub extern "C" fn vga_entry(uc:u8,color:u8)->u16{
    (uc as u16)|((color as u16)<<8)
}

const VgaWidth:usize=80;
const VgaHeight:usize=25;
const VgaMemory:usize=0xB8000;

#[repr(C)]
pub struct Terminal{
   terminal_row:usize,
   terminal_column:usize,
   terminal_color:u8,
   terminal_buffer:*mut u16
}

pub unsafe extern "C" fn strlen(s:*const u8)->usize{
   let mut size:usize=0;
   while *s.add(size)!=0{
      size+=1; 
   }
   size
}

impl Terminal{
   pub extern "C" fn default_new()->Self{
      Self{
          terminal_row:0,
          terminal_column:0,
          terminal_buffer: core::ptr::null_mut(),
          terminal_color:0
      }
   }
   pub unsafe extern "C" fn init(&mut self){
      self.terminal_row=0;
      self.terminal_column=0;
      self.terminal_buffer=VgaMemory as *mut u16;
      self.terminal_color=vga_entry_color(VgaColor::VgaColorLightGrey,VgaColor::VgaColorBlack);

      for y in 0..VgaHeight{
         for x in 0..VgaWidth{
            let index=y*VgaWidth+x;
            //write_volatile(self.terminal_buffer.add(index), vga_entry(b' ', self.terminal_color));
            *(self.terminal_buffer.add(index))=vga_entry(b' ',self.terminal_color);
         }
      }
   }
   pub extern "C" fn color(&mut self,color:u8){
      self.terminal_color=color;
   }
   pub unsafe extern "C" fn putEntryAt(&self,c:u8,color:u8,x:usize,y:usize){
      let index=y*VgaWidth+x;
      *(self.terminal_buffer.add(index))=vga_entry(c,color);
   }
   pub unsafe extern "C" fn putChar(&mut self,c:u8){
      self.terminal_column+=1;
      if self.terminal_column>=VgaWidth||c==10{
         self.terminal_column=0;
         self.terminal_row+=1;
         if self.terminal_row>=VgaHeight{
            self.terminal_row=0;
         }
      }
      self.putEntryAt(c,VgaColor::VgaColorLightCyan as u8,self.terminal_column,self.terminal_row);
   }
   pub unsafe extern "C" fn write(&mut self,data:*const u8,size:usize){
      for i in 0..size{
         self.putChar(*data.add(i));
      }
   }
   pub unsafe extern "C" fn writeString(&mut self,data:*const u8,len:usize){
      self.write(data,len);
   }
   pub unsafe fn writeHexU32(&mut self,data:u32){
      self.putChar(b'0');
      self.putChar(b'x');
      for i in (0..8).rev(){
         let nib:u8=(data>>(4*i) & 0xF).try_into().unwrap();
         let mut c:u8=if nib <10{
            nib+b'0'
         }else{
             (nib-10)+b'A'
         };
         self.putChar(c);
      }
   }
}

#[repr(C)]
struct MultibootInfo{
   flags:u32,
   mem_lower:u32,
   mem_upper:u32,
   boot_device:u32
}

#[unsafe(no_mangle)]
pub unsafe  extern "C" fn kernel_main(magic:u32,mbi:*const MultibootInfo){
   let mut terminal=Terminal::default_new();
   terminal.init();
   let msg=b"Hello, World\nGoodBye\nXDDDDDDDD";
   //terminal.writeString("Magic:".as_ptr(),"Magic:".len());
   //terminal.writeHexU32(magic); //ebx Must contain the magic value ‘0x2BADB002’; the presence of this value indicates to the operating system that it was loaded by a Multiboot-compliant boot loader (e.g. as opposed to another type of boot loader that the operating system can also be loaded from).
   //terminal.terminal_column=VgaWidth;
   //terminal.writeHexU32((*mbi).flags);
   //terminal.TerminalWriteString(msg.as_ptr(),msg.len());
   test::serial();
}




