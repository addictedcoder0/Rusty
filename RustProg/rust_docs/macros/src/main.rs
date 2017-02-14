// a simple macro similar to vec
macro_rules! veco {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
macro_rules! foo {
 	(x=> $e:expr) => (println!("mode X:{:?}", $e));
 	(y=> $e:expr) => (println!("mode Y:{:?}", $e));
}
macro_rules! combo {
 	(
 		$(
 			$x:expr; [$($y:expr),*]
 			);*
 		) => {
 			&[$($($x+$y),*),*]	
 	}
 } 
fn main() {
	let a:&[i32] = combo!(10;[1,2,3];
							20;[4,5,6]);
	println!("combo : {:?}",a );

   foo!(y=>5);
	//foo!(z=>10);error:no rules expected the token 'z'

    let x:Vec<i32> = vec![1,2,3];
    println!("x :{:?}", x);
// above vec![] is a syntactic shorthand for 
    let y = {
    	let mut temp_vec = Vec::new();
    	temp_vec.push(1);
    	temp_vec.push(2);
    	temp_vec.push(3);
    	temp_vec    };
    println!("y :{:?}",y );
//let's call the macro based version of vec! 
    let k = veco![3,4,5];
    println!("k :{:?}",k );
}




