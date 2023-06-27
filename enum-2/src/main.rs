fn main() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let some_number = Some('c');
    let absent_number: Option<i32> = None;
    impl Message {
        fn call(&self) {
            if let Message::Move { x, y } = self {
                println!("x: {}, y: {}", x, y);
            }
            // println!("Hello, {:?}", self::Move.x);
        }
    }
    let xyz = Message::Move { x: 7, y: 9 };
    xyz.call();

    /////////////////// struct move1 ////////////////////

    struct Move1 {
        x: i32,
        y: i32,
    };
    impl Move1 {
        fn call(&self) {
            println!("Hello, {:?}", self.x);
        }
    }
    let new_move = Move1 { x: 10, y: 20 };
    new_move.call();
}
