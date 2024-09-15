use allocator::Sallocator;

mod allocator;

fn main() {
    let mut sal: Sallocator<u32> = Sallocator::new().unwrap();
    sal.malloc(size_of::<u32>() * 10).unwrap();
    sal.set(9, 123).unwrap();

    sal.realloc(size_of::<u32>() * 11).unwrap();
    sal.set(11, 124).unwrap();

    println!("9: {}", sal.get(9).unwrap());
    println!("10: {}", sal.get(10).unwrap());

    sal.free().unwrap();
    sal.destroy().unwrap();
}