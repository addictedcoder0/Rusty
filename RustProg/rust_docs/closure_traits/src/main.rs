
fn main() {
    //1.FnOnce : allows the mutation of captured state, can be called once
    //2.FnMut : allows the mutation of captured state, can be called many times
    //3.Fn : don't allows the mutation of captured state, can be called many times

    /*
    FnOnce behind the scene creates a invisible structs that capture all your env var
    and a single function , this single fn will take the struct by self(ownership).
    the ownership transfer is hapenning here
    */
    
    /*
	FnMut : here struct is created + single function ,
	this function takes (&mut self)mutable reference .
    */

    /*
    Fn : here structe is created + single function ,
    this function takes (&self) immutable reference.
    */
    let state =5;
    let value = Some(6);

    println!("{:?}",value.map(|n| state+n ) );
}
