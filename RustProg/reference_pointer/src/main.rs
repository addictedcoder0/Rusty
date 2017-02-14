fn main() {
	let mut n = 42;
	//Reference out of scope => no space freeing
    {
    let m = &mut n;
   	*m = 45;
    println!("m :{:?} and address :{:p}",m,&m );
    }
    let k = &n;
    let p = &k;
    let q = &p;
    println!("n :{:?} and address :{:p}",n,&n );
    println!("k :{:?} and address :{:p}",k,&k );
   	println!("p :{:?} and address :{:p}",p,&p );
    println!("q :{:?} and address :{:p}",q,&q );
}
