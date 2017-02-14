fn main() {
    let num:i32 = 150;
    let big_num:i32 = std::i32::MAX;
    let num_wrap_add1 = num.wrapping_add(20);
	let num_wrap_add2 = big_num.wrapping_add(20);
    println!("num_wrapper_add of 150 and 20 => {:?}",num_wrap_add1 );
    println!("num_wrapper_add of int-MAX and 20 => {:?}",num_wrap_add2 );
}
