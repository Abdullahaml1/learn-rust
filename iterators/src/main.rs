fn main() {
    // -----------------------------------------------------------------------------
    // Iterator: prodly we have two types of iterators:
    // * `iter()`: borrows only (reference only items)
    // * `iter()`: borrows mutable reference only (reference only items)
    // to enable the iterator taking ownership use for example `v.iter_into()`
    // -----------------------------------------------------------------------------
    let v = vec![1, 2, 3, 4];
    let mut v_iter = v.iter();
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), Some(&4));
    let v_iter = v.iter();
    let total: i32 = v_iter.sum();
    println!("Sum {}", total);

    // moving the items into the iterator
    let mut v_iter = v.into_iter();
    assert_eq!(v_iter.next(), Some(1));
    assert_eq!(v_iter.next(), Some(2));
    assert_eq!(v_iter.next(), Some(3));
    assert_eq!(v_iter.next(), Some(4));

    // &v[0]; // BUG: v has moved with the into_iter

    enum PoolState {
        First(usize),
        Second(usize),
        Done,
    }
    struct Pool {
        pub first: Vec<i32>,
        pub second: Vec<i32>,
        state: PoolState,
    }
    impl Pool {
        fn new(first: Vec<i32>, second: Vec<i32>) -> Self {
            let mut state = PoolState::First(0);
            if first.is_empty() && !second.is_empty() {
                state = PoolState::Second(0);
            } else if second.is_empty() && first.is_empty() {
                state = PoolState::Done;
            }

            Self {
                first,
                second,
                state: state,
            }
        }

        fn iter(&self) -> impl Iterator<Item = &i32> {
            self.first.iter().chain(self.second.iter())
        }

        fn inc(&mut self) {
            match self.state {
                PoolState::First(len) => {
                    if (len + 1) < self.first.len() {
                        self.state = PoolState::First(len + 1);
                    } else if !self.second.is_empty() {
                        self.state = PoolState::Second(0);
                    } else {
                        self.state = PoolState::Done;
                    }
                }
                PoolState::Second(len) => {
                    if (len + 1) < self.second.len() {
                        self.state = PoolState::Second(len + 1)
                    } else {
                        self.state = PoolState::Done
                    }
                }
                PoolState::Done => (),
            }
        }
    }

    impl Iterator for Pool {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            match self.state {
                PoolState::Done => None,
                PoolState::First(idx) => {
                    let val = self.first[idx].clone(); // immutable borrow ends here
                    self.inc();
                    Some(val)
                }
                PoolState::Second(idx) => {
                    let val = self.second[idx].clone();
                    self.inc();
                    Some(val)
                }
            }
        }
    }

    let pool = Pool::new(vec![1, 2, 3, 4], vec![5, 6, 7, 8]);
    let pool_iter = pool.into_iter();
    for item in pool_iter {
        println!("{}", item);
    }

    // -----------------------------------------------------------------------------
    // .map() returns a new iterator with the changed values
    // -----------------------------------------------------------------------------
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|i| i * i).collect();
    println!("{:?}, {:?}", v1, v2);

    // -----------------------------------------------------------------------------
    // .filter() returns a new iterator with the changed values
    // -----------------------------------------------------------------------------
    let v1 = vec![1, 2, 3];
    let filtered_v1: Vec<_> = v1.iter().filter(|r: &&i32| *r % 2 == 0).collect(); // filterd_v1 borrowd
    // the vaulues from v1 that is subject to the filer
    println!("{:?}, {:?}", v1, filtered_v1);

    // You might wonder why the first filter uses *x and the second filter does not. v.iter() produces an Iterator<Item = &i32>. The .filter() call takes an Iterator<Item = T> as input, and passes &T to its predicate. Therefore x: &&i32 on line 3. The Rust standard library implements the remainder operator % for &i32 on the left-hand side (see the docs), but not for &&i32. So we have to dereference x once to use it in the expression *x % 2.

    //By contrast on line 4, when .map() takes an Iterator<Item = T> as input, it passes T to its closure. Therefore the closure in map takes &i32 as input. The multiplication operator * is implemented for &i32, so x does not need to be dereferenced in x * 2. The operation x * 2 produces a value of type i32, so the result of the map is an Iterator<Item = i32>. The filter then takes x : &i32, which also does not need a dereference to do x % 2. Now you know!
    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v
        .iter()
        .filter(|x: &&i32| *x % 2 == 0)
        .map(|x: &i32| x * 2)
        .collect();

    let b: Vec<_> = v
        .iter()
        .map(|x: &i32| x * 2)
        .filter(|x: &i32| x % 2 == 0)
        .collect();

    // ---------------------------------------------------------
    // Important Note: Rust Iterators like c++ have zero-runtime abstraction
    // which means for example all the map -> filter -> collect -> .... implemmented in LLVM as a
    // single fore loop with optimized not temp variables
    // ---------------------------------------------------------
}
