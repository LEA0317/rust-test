#![feature(asm)]

use std::time::Instant;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn branch_tp(range: u64) {
   let start = Instant::now();

   for _i in 0..range {
       unsafe {
   	       asm!("nop");
       }
   }

   let end = start.elapsed();
   let time = end.as_secs()*1_000_000_000 + (end.subsec_nanos() as u64);

   println!("{}", (time as f64) / (range as f64));
}

fn main() {
   branch_tp(100_000_000);
}
