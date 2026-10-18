

#[macro_export]
macro_rules! outb{
   ($port:expr,$value:expr)=>{{
       unsafe{
          core::arch::asm!(
              "out dx,al",
              in("al") $value as u8,
              in("dx") $port as u16,
              options(nomem,nostack,preserves_flags),
              );
       }
   }};
}

#[macro_export]
macro_rules! inb{
   ($port:expr)=>{{
       let mut v: u8;
       unsafe{
          core::arch::asm!(
              "in al,dx",
              in("dx") $port as u16,
              out("al") v,
              options(nomem,nostack,preserves_flags),
              );
       }
       v
   }};
}

