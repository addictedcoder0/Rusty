use std::str::FromStr;
fn pair_seperator<T>(expression:&str,separator:char)-> Option<(T,T)>  where T : FromStr 
{
	match expression.find(separator) {
		None => None,
		Some(index) => {
			match (T::from_str(&expression[..index]),T::from_str(&expression[index+1..])){
				(Ok(l),Ok(r)) => Some((l,r)),
				_ => None
			}
		}
	}
}
fn main() {
	assert_eq!(pair_seperator::<i32>("3,4",','),Some((3,4)) );
	assert_eq!(pair_seperator::<i32>("3,",','),None );
	assert_eq!(pair_seperator::<f64>("5.0x",'x'),None );
	assert_eq!(pair_seperator::<f64>("5.0x3.5",'x'),Some((5.0,3.5)) );
	assert_eq!(pair_seperator::<f64>("5.0:2.0",':'),Some((5.0,2.0)) );
}


