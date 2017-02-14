#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}
#[derive(Debug)]
enum Boardmessage {
	Move{news:&'static str},
}
fn main() {
   let msg:Message = Message::Move{x:10,y:20};
   let board_msg:Boardmessage = Boardmessage::Move{news:"Bomb attack"};
   println!("message : {:?}",msg );
   println!("board message :{:?}",board_msg );
}
