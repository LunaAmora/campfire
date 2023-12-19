pub trait RefType {
    type Type;
    type Ref<'r>
    where
        Self::Type: 'r;

    fn borrow(t: &mut Self::Type) -> Self::Ref<'_>;
}

impl<'a, T> RefType for &'a T {
    type Type = T;
    type Ref<'r> = &'r Self::Type where Self::Type: 'r;

    fn borrow(t: &mut Self::Type) -> Self::Ref<'_> {
        t
    }
}

impl<'a, T> RefType for &'a mut T {
    type Type = T;
    type Ref<'r> = &'r mut Self::Type where Self::Type: 'r;

    fn borrow(t: &mut Self::Type) -> Self::Ref<'_> {
        t
    }
}
