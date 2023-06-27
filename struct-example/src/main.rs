struct Rectangle {
    width:u32,
    hight:u32,
}

struct IpAddr {
    v4:String,
    v6:String,
}

enum IpAddrKind {
    V4,
    V6,
}



fn main() {
   let rect1 = Rectangle{
    width:30,
    hight:80,
   };

   let upendra = AadhaarCard{

   }
    println!(
        "The area of the rectangle having hight {} and width {}  is {} square pixels.",
        rect1.hight ,rect1.width , area(&rect1)
    ); 

    updateAaadar(upendra)
}
fn area(rectangle:&Rectangle) -> u32 {
    rectangle.width * rectangle.hight
}


struct AadhaarCard {
    name: String,
    gender: String,
    date_of_birth: String,
    aadhaar_number: String,
    address: String,
}

impl AadhaarCard{

fn createuserofaadar(user:AadhaarCard){

}

fn updateAaadar(user:AadhaarCard){

}


}