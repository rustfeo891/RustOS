pub const COM1:u16=0x3F8;

pub fn init()->Result<(),()>{
   crate::outb!(COM1+1,0x00);
   crate::outb!(COM1+3,0x80);
   crate::outb!(COM1+0,0x03);
   crate::outb!(COM1+1,0x00);
   crate::outb!(COM1+3,0x03);
   crate::outb!(COM1+2,0xC7);
   crate::outb!(COM1+4,0x0B);
   crate::outb!(COM1+4,0x01E);
   crate::outb!(COM1+0,0xAE);
   if crate::inb!(COM1+0)!=0xAE{
      return Err(());
   }
   crate::outb!(COM1+4,0x0F);
   Ok(())
}

pub fn received()->bool{
   crate::inb!(COM1+5)&1 != 0
}

pub fn read()->u8{
   //while !received() {}
   crate::inb!(COM1)
}

pub fn transmitEmpty()->bool{
   crate::inb!(COM1+5)&0x20 !=0
}

pub fn write(a:u8){
   //while !transmitEmpty(){}
   crate::outb!(COM1,a)
}

