use crate::{new_data, Component, EntityData};

pub trait RefType {
    type Type;
    type Ref<'r>
    where
        Self::Type: 'r;

    fn borrow(t: &mut Self::Type) -> Self::Ref<'_>;
    fn mutate(_: Self::Type, _: &mut EntityData)
    where
        Self::Type: Component,
    {
    }
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

    fn mutate(t: Self::Type, data: &mut EntityData)
    where
        Self::Type: Component,
    {
        let (id, comp) = new_data(t);
        data.insert(id, comp);
    }
}
