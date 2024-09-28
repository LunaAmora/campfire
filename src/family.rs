pub trait RefType {
    const IS_MUT: bool;

    type Type;
    type Ref<'r>
    where
        Self::Type: 'r;

    fn borrow(t: &mut Self::Type) -> Self::Ref<'_>;
}

impl<'a, T> RefType for &'a T {
    const IS_MUT: bool = false;

    type Type = T;
    type Ref<'r>
        = &'r Self::Type
    where
        Self::Type: 'r;

    fn borrow(t: &mut Self::Type) -> Self::Ref<'_> {
        t
    }
}

impl<'a, T> RefType for &'a mut T {
    const IS_MUT: bool = true;

    type Type = T;
    type Ref<'r>
        = &'r mut Self::Type
    where
        Self::Type: 'r;

    fn borrow(t: &mut Self::Type) -> Self::Ref<'_> {
        t
    }
}
