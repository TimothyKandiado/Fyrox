//! Test cases for `fyrox_core::reflect::Reflect`

use std::ops::{Deref, DerefMut};

use fyrox_core::reflect::*;

#[allow(dead_code)]
#[derive(Reflect)]
pub struct Struct {
    field: usize,
    #[reflect(hidden)]
    hidden: usize,
}

#[allow(dead_code)]
#[derive(Reflect)]
pub struct Tuple(usize, usize);

#[allow(dead_code)]
#[derive(Reflect)]
pub enum Enum {
    Named { field: usize },
    Tuple(usize),
    Unit,
}

#[test]
fn property_constants() {
    assert_eq!(Struct::FIELD, "field");

    // hidden fields don't expose their keys
    // assert_eq!(SStruct::HIDDEN, "hidden");

    assert_eq!(Tuple::F_0, "0");
    assert_eq!(Tuple::F_1, "1");

    assert_eq!(Enum::NAMED_FIELD, "Named@field");
    assert_eq!(Enum::TUPLE_F_0, "Tuple@0");
}

#[test]
fn reflect_field_accessors() {
    let mut s = Struct {
        field: 10,
        hidden: 10,
    };

    assert_eq!(s.get_field::<usize>(Struct::FIELD), Some(&10));
    assert_eq!(s.get_field_mut::<usize>(Struct::FIELD), Some(&mut 10));
    *s.get_field_mut::<usize>(Struct::FIELD).unwrap() = 100;
    assert_eq!(s.get_field::<usize>(Struct::FIELD), Some(&100));

    assert!(s.get_field::<usize>("HIDDEN").is_none());

    let mut t = Tuple(0, 1);

    assert_eq!(t.get_field::<usize>(Tuple::F_0), Some(&0));
    assert_eq!(t.get_field::<usize>(Tuple::F_1), Some(&1));

    *t.get_field_mut::<usize>(Tuple::F_0).unwrap() += 10;
    *t.get_field_mut::<usize>(Tuple::F_1).unwrap() += 10;

    assert_eq!(t.get_field::<usize>(Tuple::F_0), Some(&10));
    assert_eq!(t.get_field::<usize>(Tuple::F_1), Some(&11));

    let mut e_named = Enum::Named { field: 10 };

    assert_eq!(e_named.get_field::<usize>(Enum::NAMED_FIELD), Some(&10));
    assert_eq!(e_named.get_field::<usize>(Enum::TUPLE_F_0), None);
    *e_named.get_field_mut::<usize>(Enum::NAMED_FIELD).unwrap() = 20usize;
    assert_eq!(e_named.get_field::<usize>(Enum::NAMED_FIELD), Some(&20));

    let mut e_tuple = Enum::Tuple(30);

    assert_eq!(e_tuple.get_field::<usize>(Enum::NAMED_FIELD), None);
    assert_eq!(e_tuple.get_field::<usize>(Enum::TUPLE_F_0), Some(&30));
    *e_tuple.get_field_mut::<usize>(Enum::TUPLE_F_0).unwrap() = 40usize;
    assert_eq!(e_tuple.get_field::<usize>(Enum::TUPLE_F_0), Some(&40));
}

#[test]
fn reflect_containers() {
    struct DerefContainer<T> {
        data: T,
    }

    impl<T> Deref for DerefContainer<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<T> DerefMut for DerefContainer<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
    }

    #[derive(Reflect)]
    struct X {
        #[reflect(deref)]
        container: DerefContainer<Struct>,
    }

    let x = X {
        container: DerefContainer {
            data: Struct {
                field: 0,
                hidden: 1,
            },
        },
    };

    assert!(x
        .cast_resolve_path::<usize>("container.data.field")
        .is_err());

    assert_eq!(x.cast_resolve_path::<usize>("container.field"), Ok(&0));

    #[derive(Reflect)]
    #[reflect(bounds = "T: Reflect")]
    struct B<T> {
        #[reflect(deref)]
        data: Box<T>,
    }

    let b = B {
        data: Box::new(Struct {
            field: 10,
            hidden: 11,
        }),
    };

    assert_eq!(b.cast_resolve_path::<usize>("data.field"), Ok(&10));
    assert!(x.cast_resolve_path::<usize>("data.hidden").is_err());
}

#[test]
fn reflect_path() {
    #[derive(Reflect)]
    struct Hierarchy {
        s: Struct,
        e: Enum,
    }

    let mut hie = Hierarchy {
        s: Struct {
            field: 1,
            hidden: 2,
        },
        e: Enum::Tuple(10),
    };

    assert_eq!(hie.cast_resolve_path::<usize>("s.field"), Ok(&1));
    assert_eq!(hie.cast_resolve_path::<usize>("e.Tuple@0"), Ok(&10));

    assert_eq!(hie.cast_resolve_path_mut::<usize>("s.field"), Ok(&mut 1));
    assert_eq!(hie.cast_resolve_path_mut::<usize>("e.Tuple@0"), Ok(&mut 10));
}
