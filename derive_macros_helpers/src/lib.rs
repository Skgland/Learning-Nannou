pub use bounded::Bounded;
pub use enumerable::{Enumerable, EnumerableIterator};

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

    macro_rules! impl_nums_bounded {
        ($a:ident) => {
            impl Bounded for $a {
                fn minimum() -> Self {
                    $a::MIN
                }

                fn maximum() -> Self {
                    $a::MAX
                }
            }
        };
    }

    impl_nums_bounded!(u8);
    impl_nums_bounded!(i8);
    impl_nums_bounded!(u16);
    impl_nums_bounded!(i16);
    impl_nums_bounded!(u32);
    impl_nums_bounded!(i32);
    impl_nums_bounded!(u64);
    impl_nums_bounded!(i64);
    impl_nums_bounded!(u128);
    impl_nums_bounded!(i128);
}

mod enumerable {
    use super::bounded::Bounded;

    pub trait Enumerable
    where
        Self: Sized,
    {
        fn next(&self) -> Option<Self>;

        fn enumerate_all() -> EnumerableIterator<Self>
        where
            Self: Bounded,
        {
            EnumerableIterator {
                current: Some(Self::minimum()),
            }
        }

        fn enumerate_following(&self) -> EnumerableIterator<Self> {
            EnumerableIterator {
                current: self.next(),
            }
        }
    }

    pub struct EnumerableIterator<A> {
        current: Option<A>,
    }

    impl<A> Iterator for EnumerableIterator<A>
    where
        A: Enumerable,
    {
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
            } else {
                Some(true)
            }
        }
    }
}

macro_rules! impl_nums_enumerable {
    ($a:ident) => {
        impl Enumerable for $a {
            fn next(&self) -> Option<Self> {
                self.checked_add(1)
            }
        }
    };
}

impl_nums_enumerable!(u8);
impl_nums_enumerable!(i8);
impl_nums_enumerable!(u16);
impl_nums_enumerable!(i16);
impl_nums_enumerable!(u32);
impl_nums_enumerable!(i32);
impl_nums_enumerable!(u64);
impl_nums_enumerable!(i64);
impl_nums_enumerable!(u128);
impl_nums_enumerable!(i128);
