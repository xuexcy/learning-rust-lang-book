pub mod animal {
    pub mod dog {
        pub trait Dog {
            fn species(&self) {}
        }
        pub struct Corgi;
        impl Dog for Corgi {
            fn species(&self) {
                println!("Corgi");
            }
        }
    }

    mod cat {}
    mod finish {}
}
