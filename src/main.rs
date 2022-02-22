pub mod core;

fn main() {
    let p = core::point::Point::broadcast(0.0);

    println!("Hello, world!{:?}",p);
}
