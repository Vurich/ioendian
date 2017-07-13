use std::fmt::{Debug, Display, Formatter, Error};

pub trait EndianBufFor<Out> {
    fn reverse(&mut self);
    fn native(self) -> Out;
}

pub trait FromBuf: Sized + Clone {
    type Buf: EndianBufFor<Self>;

    fn from_buf(buf: Self::Buf) -> Self {
        buf.native()
    }
}

pub trait IntoNativeEndian {
    type Out;
    fn native(self) -> Self::Out;
}

#[derive(Clone)]
pub struct Big<T: FromBuf>(pub T::Buf);
#[derive(Clone)]
pub struct Little<T: FromBuf>(pub T::Buf);

impl<T: FromBuf + Debug> Debug for Big<T> where Self: Clone {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Big({:?})", self.clone().native())
    }
}

impl<T: FromBuf + Debug> Debug for Little<T> where Self: Clone {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Big({:?})", self.clone().native())
    }
}

impl<T: FromBuf + Display> Display for Big<T> where Self: Clone {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.clone().native().fmt(f)
    }
}

impl<T: FromBuf + Debug> Display for Little<T> where Self: Clone {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.clone().native().fmt(f)
    }
}

macro_rules! impl_buf_traits {
    ($type:ty, $count:expr) => {
        impl EndianBufFor<$type> for [u8; $count] {
            fn reverse(&mut self) {
                self.as_mut().reverse()
            }
            fn native(self) -> $type {
                unsafe { ::std::mem::transmute(self) }
            }
        }

        impl FromBuf for $type {
            type Buf = [u8; $count];
        }

        impl Copy for Big<$type> {}
        impl Copy for Little<$type> {}
    };
}

impl_buf_traits!(u8, 1);
impl_buf_traits!(u16, 2);
impl_buf_traits!(u32, 4);
impl_buf_traits!(u64, 8);

impl_buf_traits!(i8, 1);
impl_buf_traits!(i16, 2);
impl_buf_traits!(i32, 4);
impl_buf_traits!(i64, 8);

#[cfg(target_endian = "big")]
mod trait_impls {
    use super::{FromBuf, EndianBufFor, IntoNativeEndian, Big, Little};

    impl<T: FromBuf> IntoNativeEndian for Big<T> {
        type Out = T;

        #[inline(always)]
        fn native(self) -> Self::Out {
            T::from_buf(self.0)
        }
    }

    impl<T: FromBuf> IntoNativeEndian for Little<T> {
        type Out = T;

        #[inline(always)]
        fn native(mut self) -> Self::Out {
            self.0.reverse();
            T::from_buf(self.0)
        }
    }
}

#[cfg(target_endian = "little")]
mod trait_impls {
    use super::{FromBuf, EndianBufFor, IntoNativeEndian, Big, Little};

    impl<T: FromBuf> IntoNativeEndian for Big<T> {
        type Out = T;

        #[inline(always)]
        fn native(mut self) -> Self::Out {
            self.0.reverse();
            T::from_buf(self.0)
        }
    }

    impl<T: FromBuf> IntoNativeEndian for Little<T> {
        type Out = T;

        #[inline(always)]
        fn native(self) -> Self::Out {
            T::from_buf(self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
