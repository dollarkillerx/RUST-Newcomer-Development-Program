trait Foo<'a> {}

struct FooImpl<'a> {
    s: &'a [u32],
}

impl<'a> Foo<'a> for FooImpl<'a> {}

// 'a > 'b
fn foo<'b,'a: 'b>(s: &'a [u32]) -> Box<dyn Foo<'b> + 'b> {
    Box::new(FooImpl { s })
}

fn main() {
    println!("Hello, world!");
}
