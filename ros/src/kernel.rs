#![no_std]
#![no_main]

use core::ops::Add;
use core::panic::PanicInfo;
use core::ptr::write_volatile;

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
   pub unsafe extern "C" fn TerminalInit(&mut self){
      self.terminal_row=0;
      self.terminal_column=0;
      self.terminal_buffer=VgaMemory as *mut u16;
      self.terminal_color=vga_entry_color(VgaColor::VgaColorLightGrey,VgaColor::VgaColorBlack);

      for y in 0..VgaHeight{
         for x in 0..VgaWidth{
            let index=y*VgaWidth+x;
            write_volatile(self.terminal_buffer.add(index), vga_entry(b' ', self.terminal_color));
            //*(self.terminal_buffer.add(index))=vga_entry(b' ',self.terminal_color);
         }
      }
   }
   pub extern "C" fn TerminalColor(&mut self,color:u8){
      self.terminal_color=color;
   }
   pub unsafe extern "C" fn TerminalPutEntryAt(&self,c:u8,color:u8,x:usize,y:usize){
      let index=y*VgaWidth+x;
      *(self.terminal_buffer.add(index))=vga_entry(c,color);
   }
   pub unsafe extern "C" fn TerminalPutChar(&mut self,c:u8){
      self.TerminalPutEntryAt(c,self.terminal_color,self.terminal_column,self.terminal_row);
      self.terminal_column+=1;
      if self.terminal_column>=VgaWidth{
         self.terminal_column=0;
         self.terminal_row+=1;
         if self.terminal_row>=VgaHeight{
            self.terminal_row=0;
         }
      }
   }
   pub unsafe extern "C" fn TerminalWrite(&mut self,data:*const u8,size:usize){
      for i in 0..size{
         self.TerminalPutChar(*data.add(i));
      }
   }
   pub unsafe extern "C" fn TerminalWriteString(&mut self,data:*const u8,len:usize){
      self.TerminalWrite(data,len);
   }
}

#[unsafe(no_mangle)]
pub unsafe  extern "C" fn kernel_main(){
   let mut terminal=Terminal::default_new();
   terminal.TerminalInit();
   let msg=b"Hello, World\n";
   terminal.TerminalWriteString(msg.as_ptr(),msg.len());
}

