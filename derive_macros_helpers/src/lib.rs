pub trait Bounded {
    fn minimum() -> Self;
    fn maximum() -> Self;
}

pub trait Enumerable where Self:Sized{
    fn next(&self) -> Option<Self>;

    fn first() -> Self where Self:Bounded {
        Self::minimum()
    }

    fn iter() -> EnumerableIterator<Self> where Self:Bounded{
        EnumerableIterator{current:Some(Self::first())}
    }
}

pub struct EnumerableIterator<A> {
    current: Option<A>
}

impl <A> Iterator for EnumerableIterator<A> where A:Bounded+Enumerable{
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(current) = self.current.take(){
            if let Some(next) = current.next() {
                self.current.replace(next);
            }
            Some(current)
        }else{
            None
        }

    }
}