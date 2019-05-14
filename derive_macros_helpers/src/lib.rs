pub use bounded::Bounded;
pub use enumerable::{Enumerable,EnumerableIterator};

mod bounded {
    pub trait Bounded {
        fn minimum() -> Self;
        fn maximum() -> Self;
    }

    impl Bounded for bool {
        fn minimum() -> Self {
            false
        }

        fn maximum() -> Self {
            true
        }
    }

    macro_rules! impl_nums {
        ($a:ident) => {
            impl Bounded for $a {
                fn minimum() -> Self {
                    std::$a::MIN
                }

                fn maximum() -> Self {
                    std::$a::MAX
                }
            }
        };
    }

    impl_nums!(u8);
    impl_nums!(i8);
    impl_nums!(u16);
    impl_nums!(i16);
    impl_nums!(u32);
    impl_nums!(i32);
    impl_nums!(u64);
    impl_nums!(i64);
    impl_nums!(u128);
    impl_nums!(i128);

}

mod  enumerable {
    use super::bounded::Bounded;

    pub trait Enumerable where Self: Sized {
        fn next(&self) -> Option<Self>;

        fn first() -> Self where Self: Bounded {
            Self::minimum()
        }

        fn reset(&self) -> Self where Self:Bounded {
            Self::first()
        }

        fn enumerate_all() -> EnumerableIterator<Self> where Self: Bounded {
            EnumerableIterator { current: Some(Self::first()) }
        }

        fn enumerate_following(&self) -> EnumerableIterator<Self>  {
            EnumerableIterator{ current:self.next()}
        }
    }

    pub struct EnumerableIterator<A> {
        current: Option<A>
    }

    impl<A> Iterator for EnumerableIterator<A> where A: Enumerable {
        type Item = A;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(current) = self.current.take() {
                if let Some(next) = current.next() {
                    self.current.replace(next);
                }
                Some(current)
            } else {
                None
            }
        }
    }

    impl Enumerable for bool {
        fn next(&self) -> Option<Self> {
            if *self {
                None
            }
            else{
                Some(true)
            }
        }
    }
}
