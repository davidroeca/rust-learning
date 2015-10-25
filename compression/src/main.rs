mod structs;
use structs::Heap;

fn main() {
    let mut n = Heap::new(1, None, None);
    println!("{}", n.val);
}
