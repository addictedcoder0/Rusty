fn main() {
   let x = 5;
   let raw = &x as *const i32;
   unsafe{
   		println!("*const reference: {:?}", *raw);
   }
   let mut y = 10;
   let raw_mut = &mut y as *mut i32;
   unsafe{
   		*raw_mut+=5;
   		println!("*mut reference: {:?}", *raw_mut);
   }
}

