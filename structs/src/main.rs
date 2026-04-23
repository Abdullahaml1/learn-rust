#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_hamo(email: String) -> User {
    User {
        active: true,
        username: String::from("Hamo"),
        email: email,
        sign_in_count: 0,
    }
}

//------------------------------------------------------
// Tuple Struct
//------------------------------------------------------
#[derive(Debug)]
struct Color(i32, i32, i32);

//------------------------------------------------------
// Unit Struct
//------------------------------------------------------
#[derive(Debug)]
struct AlwaysEequal;

//------------------------------------------------------
// Example of Structs
//------------------------------------------------------
#[derive(Debug)]
struct Rectangle {
    width: u32,
    hight: u32,
}

fn cal_areal(rect: &Rectangle) -> u64 {
    rect.width as u64 * rect.hight as u64
}

//------------------------------------------------------
// Using Methods
//------------------------------------------------------
#[derive(Debug)]
struct Square {
    width: u32,
    hight: u32,
}

impl Square {
    fn new(w: u32, h: u32) -> Self {
        // associated function
        Self { width: w, hight: h }
    }
    fn width(&self) -> bool {
        // checking if width >0
        self.width > 0
    }
    fn hight(&self) -> bool {
        // checking if hight >0
        self.hight > 0
    }
    fn area(&self) -> u64 {
        if self.width() && self.hight() {
            self.width as u64 * self.hight as u64
        } else {
            0
        }
    }
    fn can_hold(&self, other: &Square) -> bool {
        self.width >= other.width && self.hight >= other.hight
    }
}

fn main() {
    let email = String::from("hamo.com");
    let user1 = build_hamo(email);
    println!("User is: {:?}", user1);

    let user2 = User {
        username: String::from("Nono"),
        ..user1
    };
    println!("User is: {:?}", user2);
    // All the fields of user1 is not valid exept for username
    println!("User is: {}", user1.username);

    //------------------------------------------------------
    // Unit Struct
    //------------------------------------------------------
    let black = Color(0, 0, 0);
    println!("The color is: {:?}", black);

    //------------------------------------------------------
    // Unit Struct
    //------------------------------------------------------
    let subject = AlwaysEequal;
    println!("The subject is: {:?}", subject);

    //------------------------------------------------------
    // Rect Exmaple
    //------------------------------------------------------
    let rect1 = Rectangle {
        width: 30,
        hight: 40,
    };
    let area = cal_areal(&rect1);
    println!("The area of {:?} is: {}", rect1, area);
    // another way of debug printing is add `#` adds line for every field
    println!("The area of {:#?}\n is: {}", rect1, area);

    // dbg! macro takes the ownershipe of the input variables to we pass it by refrence not by
    // value
    // The proper way to use it is to pass varialbes by refrences
    dbg!(&rect1);
    dbg!(&rect1);
    dbg!(&rect1);
    dbg!(rect1); // <------ Taking ownershipe over rect1
    // dbg!(rect1); // ------- Invalid

    //------------------------------------------------------
    // Using Methods
    //------------------------------------------------------
    let seq1 = Square {
        width: 50,
        hight: 50,
    };
    println!("The area of square: {:?} is {}", seq1, seq1.area());
    let seq2 = Square::new(30, 50);
    println!("{:?} can hold {:?} is {}", seq1, seq2, seq1.can_hold(&seq2));
}
