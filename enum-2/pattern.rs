fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let xyz = Message::Move { x: 7, y: 9 };
    println!("Hello, {:?}", xyz);

    if let Message::Move { x, y } = xyz {
        println!("x: {}, y: {}", x, y);
    }


    ///////// or //////////////
    
    
match xyz {
    Message::Quit => {
        println!("Received a Quit message");
    }
    Message::Move { x, y } => {
        println!("Received a Move message. x: {}, y: {}", x, y);
    }
    Message::Write(text) => {
        println!("Received a Write message: {}", text);
    }
    Message::ChangeColor(r, g, b) => {
        println!("Received a ChangeColor message. r: {}, g: {}, b: {}", r, g, b);
    }
}
}
