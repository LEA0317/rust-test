fn do_it(a :i32,
         b :i32,
	 c :&mut i32) -> i32 {
	 *c = a + b;
	 *c
}
	
fn main() {
   let mut c = 0;
   let p = &mut c;
   println!("{}", do_it(4, 5, p));
}
