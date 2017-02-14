fn main() {
	let vec1 = vec![1,2,3];
	let vec2 = vec![4,5,6];
	//find() for vec yields '&&i32'.Destructure to 'i32'.
	println!("2 in vec1: {:?}",vec1.iter().find(|&&x| x==2) ); 
	//into_iter() for vec yields '&i32'.
	println!("2 in vec2: {:?}",vec2.into_iter().find(|&x| x==2) ); 

	let arr1 = [1,2,3];
	let arr2 = [4,5,6];
	//iter() for array yields '&&i32'.
	println!("2 in arr1: {:?}",arr1.iter().find(|&&x| x==2) ); 
	//into_iter() for array , usually yields '&&i32'.
	println!("2 in arr2: {:?}",arr2.into_iter().find(|&&x| x==2) ); 
}
