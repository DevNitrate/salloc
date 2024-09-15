use allocator::Sallocator;

mod allocator;

fn main() {
    let mut sal: Sallocator<u32> = Sallocator::new(12);
    sal.salloc();
    sal.set(1, 123);
    println!("{}", sal.get(1));
    sal.free();
}