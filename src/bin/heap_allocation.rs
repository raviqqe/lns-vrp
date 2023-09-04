use std::rc::Rc;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let _profiler = dhat::Profiler::new_heap();

    for _ in 0..10 {
        let xs = vec![0usize; 1000];
        let _: Rc<[usize]> = xs.into();
    }
}
