fn main() {
    // -------------------------------------------------------
    // List of objects as a generic type similar to Sequence in python
    // -------------------------------------------------------
    fn largest<T>(list: &[T]) -> &T {
        let mut largest_num = &list[0];
        for item in list {
            if item > largest_num {
                largest_num = item;
            }
        }
        largest_num
    }
    let vec1 = vec![1, 2, 3, 9, 1];
    println!("largest: {}", largest(&vec1));
    let mut arr: [i32; 4] = [1, 3, 33, 9];
    println!("largest: {}", largest(&arr));

    // ---------------------------------------------------------------------------
    // Putting restrictins on generics
    // ---------------------------------------------------------------------------
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Self { x: x, y: y }
        }
    }

    impl Point<f32> {
        fn pow(&self) -> f32 {
            self.x.powf(self.y)
        }
    }
    let point = Point::new(1.5, 2.0);
    point.pow();
    let point = Point::new(1, 2);
    // point.pow(); //BUG: no implimentation found for pow of integer
}
