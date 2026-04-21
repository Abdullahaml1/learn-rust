#[derive(Debug)]
struct Credit {
    remaining: u64,
    bytes: usize,
}

impl Credit {
    pub fn new(x: u64) -> Self {
        Self {
            remaining: x,
            bytes: (x / 8) as usize,
        }
    }

    pub fn inc(&mut self, added: u64) {
        self.remaining += added;
        self.bytes = (self.remaining / 8) as usize;
    }
}
fn main() {
    let mut credit = Credit::new(18);
    credit.inc(2);
    print!("Credict {:?}\n", credit)
}
