use std::fmt::{Debug, Display, Formatter, Error};

pub trait EndianBufFor<Out> {
    #[inline(always)]
    fn reverse(&mut self);
    #[inline(always)]
    fn native(self) -> Out;
}

pub trait FromBuf: Sized + Clone {
    type Buf: EndianBufFor<Self>;

    #[inline(always)]
    fn from_buf(buf: Self::Buf) -> Self {
        buf.native()
    }

    #[inline(always)]
    fn into_buf(self) -> Self::Buf;
}

pub trait IntoNativeEndian {
    type Out;

    #[inline(always)]
    fn native(self) -> Self::Out;
}

#[derive(Clone)]
pub struct Big<T: FromBuf>(pub T::Buf);
#[derive(Clone)]
pub struct Little<T: FromBuf>(pub T::Buf);

impl<T: FromBuf + Debug> Debug for Big<T>
    where Self: Clone
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Big({:?})", self.clone().native())
    }
}

impl<T: FromBuf + Debug> Debug for Little<T>
    where Self: Clone
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Big({:?})", self.clone().native())
    }
}

impl<T: FromBuf + Display> Display for Big<T>
    where Self: Clone
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.clone().native().fmt(f)
    }
}

impl<T: FromBuf + Debug> Display for Little<T>
    where Self: Clone
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.clone().native().fmt(f)
    }
}

macro_rules! impl_buf_traits {
    ($type:ty, $count:expr) => {
        impl EndianBufFor<$type> for [u8; $count] {
            #[inline(always)]
            fn reverse(&mut self) {
                self.as_mut().reverse()
            }

            #[inline(always)]
            fn native(self) -> $type {
                unsafe { ::std::mem::transmute(self) }
            }
        }

        impl FromBuf for $type {
            type Buf = [u8; $count];

            #[inline(always)]
            fn into_buf(self) -> Self::Buf {
                unsafe { ::std::mem::transmute(self) }
            }
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

impl_buf_traits!(f32, 4);
impl_buf_traits!(f64, 8);

#[cfg(target_endian = "big")]
mod trait_impls {
    use super::{FromBuf, EndianBufFor, IntoNativeEndian, Big, Little};

    impl<T: FromBuf> Big<T> {
        #[inline(always)]
        fn new(inner: T) -> Self {
            Big(inner.into_buf())
        }
    }

    impl<T: FromBuf> Little<T> {
        #[inline(always)]
        fn new(inner: T) -> Self {
            let mut buf = inner.into_buf();
            buf.reverse();
            Little(buf)
        }
    }

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

    impl<T: FromBuf> Big<T> {
        #[inline(always)]
        pub fn new(inner: T) -> Self {
            let mut buf = inner.into_buf();
            buf.reverse();
            Big(buf)
        }
    }

    impl<T: FromBuf> Little<T> {
        #[inline(always)]
        pub fn new(inner: T) -> Self {
            Little(inner.into_buf())
        }
    }

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
