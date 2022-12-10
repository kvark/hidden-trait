mod hidden {
    trait Foo {
        //type Goal;
        const GOAL: f32;
        fn foo(&self) -> u32;
    }

    pub struct Bar;

    #[hidden_trait::expose]
    impl Foo for Bar {
        //type Goal = f32;
        const GOAL: f32 = 1.0;
        fn foo(&self) -> u32 {
            42
        }
    }
}

fn main() {
    let bar = hidden::Bar;
    // calling the trait method as if it's ours
    bar.foo();
    let _ = hidden::Bar::GOAL;
}
