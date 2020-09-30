struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 5 {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}

fn main() {
    let c = Counter::new();
    for i in c {
        println!("{}", i);
    }
}
