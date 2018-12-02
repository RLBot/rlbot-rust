//! Mirror the std::convert::From hierarchy twice. The goal of this is to let us
//! disambiguate between `na::Point3` and `na::Vector3` in the `DesiredPhysics`
//! setters.

pub trait FromPoint3<T> {
    fn from(_: T) -> Self;
}

impl<T> FromPoint3<T> for T {
    fn from(t: T) -> T {
        t
    }
}

pub trait Point3Into<T> {
    fn into(self) -> T;
}

impl<T, U> Point3Into<U> for T
where
    U: FromPoint3<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}

pub trait FromVector3<T> {
    fn from(_: T) -> Self;
}

impl<T> FromVector3<T> for T {
    fn from(t: T) -> T {
        t
    }
}

pub trait Vector3Into<T> {
    fn into(self) -> T;
}

impl<T, U> Vector3Into<U> for T
where
    U: FromVector3<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
