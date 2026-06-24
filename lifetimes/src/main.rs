use std::fmt::Display;

fn main() {
    //------------------------------------------------------------------
    // NOTE: lifetimes are for references not for variables
    //------------------------------------------------------------------
    //
    // In functions
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    //------------------------------------------------------------------
    // In struct definitions
    //------------------------------------------------------------------
    #[derive(Debug)]
    struct ImportantExpert<'a> {
        part: &'a str,
    }
    let text = String::from("How are you today? are you fine?");
    let first_sent = text.split('?').next().unwrap();
    let i = ImportantExpert { part: first_sent };

    let sent = i.part;
    drop(i);
    println!("{}", sent);
    // println!("{:?}", i);
    // drop(text); //BUG: the lifetimeof sent is tied to the text once droped  we can not use sent
    println!("{}", sent);

    // -----------------------------------------------------------
    // Lifetime elision (elision rules by the compiler) (deletion):
    // In some situations some patterns are deterministic and can be infred (so we have to write
    // ithem by our hands)
    // -----------------------------------------------------------
    fn get_first_word(text: &str) -> &str {
        text.split(' ').next().unwrap()
    }
    // the code complies with not concerts about the lifetimes
    println!("First word is `{}`", get_first_word("Hello how are you?"));

    // -----------------------------------------------------------
    // Lifetimes for input paramters are called `input lifeteims`
    // Lifetimes for returns are called `output lifeteims`
    // Rules of lifetime inference by the compiler (3)
    // 1. Every input paramter has differrent lifetime
    // 2. If exacally **one** input lifetime the same lifetime is assigined to **all** output
    //    lifetimes parameters.
    // ```
    // fn first_word(s: &str) -> &str;
    // fn first_word<'a>(s: &'a str) -> &'a str;
    // ```
    //
    // 3. If the one of the input lifetime is `&self` or `&mut self` the self lifeteime is assigned
    //    to all the output lifetimes
    // -----------------------------------------------------------

    #[derive(Debug)]
    struct Square {
        x: u64,
        z: f32,
    }
    impl Square {
        fn new(x: u64, z: f32) -> Self {
            Self { x, z }
        }
        // Rule3 all output have the same lifetime
        fn get_many_inputs(&mut self) -> (&u64, &f32) {
            (&self.x, &self.z)
        }
    }
    let mut s = Square::new(3, 10.1);
    println!("{:?}", s);
    let (x, z) = s.get_many_inputs();
    println!("{}, {}", x, z);
    drop(s);
    // println!("{}, {}", x, z); // BUG: error because we moved the s variable

    // ---------------------------------------------------------------
    // Using lifetimes in methods
    // ---------------------------------------------------------------
    struct Object<'a> {
        name: &'a str,
    }
    impl<'a> Object<'a> {
        fn hello(&'a mut self, s: &'a str) -> &'a str {
            self.name = s;
            self.name
        }
    }
    // ---------------------------------------------------------------
    // 'static lifetime leave the entire of the program and is stored in the data segment of the
    // binary
    // ---------------------------------------------------------------
    let s: &'static str = "Hellow world";

    //----------------------------------------------------------------
    // Generics, traits bounts and lifetimes
    //----------------------------------------------------------------
    fn longes_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() { x } else { y }
    }
}
