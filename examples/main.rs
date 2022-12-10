mod hidden {
    trait Foo {
        fn foo(&self) -> u32;
    }

    pub struct Bar;

    #[hidden_trait::expose]
    impl Foo for Bar {
        fn foo(&self) -> u32 {
            42
        }
    }
}

fn main() {
    let bar = hidden::Bar;
    // calling the trait method as if it's ours
    bar.foo();
}
