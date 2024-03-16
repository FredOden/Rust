struct Foo<'a> {
    i:&'a i32
}

fn main() {
    let x = 42;
    let y = if x == 42 {({12})}
    let foo = Foo {
        i: &x
    };
    println!("{}",foo.i);
}

