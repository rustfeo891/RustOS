pub fn serial(){
   let vga=0xB800 as *mut u16;
   match crate::serial::init(){
      Ok(())=>{}
      Err(())=>{
        //let vga=0xB800 as *mut u16;
        let msg=b"SER FAIL";
        for (i,&c) in msg.iter().enumerate(){
          unsafe{*vga.add(i)=0x0C00|c as u16;}
        }
        loop{}
      }
   }
   let mut mark= |s:&[u8],row: usize|{
     for (i,&c) in s.iter().enumerate(){
       unsafe {*vga.add(row*80+i)=0x0F00| c as u16;}
    }
  };
   let msg=b"SERIAL OK\n";
   for &b in msg{
      crate::serial::write(b);
   }

   let magic:u32=0x2BADB002;
   for i in (0..8).rev(){
      let nib=((magic>>(4*i)&0xF)) as u8;
      let c= if nib<10 {b'0'+nib} else {nib-10+b'A'};
      crate::serial::write(c);
   }
   crate::serial::write(b'\n');
}
