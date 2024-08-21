struct Data {
    #[cfg(feature = "a")]
    a: usize,
}

struct Root {
    #[cfg(feature = "a")]
    data: Data,
}

impl Root {
    #[cfg(feature = "a")]
    pub fn init() -> Self {
        Self { data: Data { a: 0 } }
    }

    #[cfg(not(feature = "a"))]
    pub fn init() -> Self {
        Self { data: Data }
    }
}

fn main() {
    let mut root = Root::init();

    #[cfg(feature = "a")]
    {
        root.data.a += 1;
    };
}

fn test(#[cfg(feature = "a")] param0: i32) {}
