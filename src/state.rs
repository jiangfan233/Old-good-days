use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::{
    persist::{Persist, PersistedOrigin},
    ref_container::RefContainer,
};

#[derive(Debug)]
pub struct State<T> {
    container: RefContainer<Option<T>>,
}

impl<T> State<T> {
    pub fn value(&self) -> Ref<'_, T> {
        Ref::map(self.container.current(), |v| {
            v.as_ref().expect("Cannot get state!!!!!")
        })
    }

    pub fn set(&mut self, f: impl FnOnce(T) -> T) {
        let val = self.container.current_mut().take();
        let new_v = val.map(f);
        self.container.set_current(new_v);
    }   

}

pub fn use_state<T>(f: impl FnOnce() -> T) -> State<T> {

    State{
        container: RefContainer::new(Some(f()))
    }
}

impl<T: 'static> Persist for State<T> {
    fn ptr(&self) -> PersistedOrigin {
        PersistedOrigin
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            container: self.container.clone(),
        }
    }
}

mod test {
    use super::{State, use_state};

    #[derive(Debug, PartialEq, Eq)]
    struct Person {
        age: i32,
        families: Vec<Person>,
    }

    #[test]
    fn test() {
        let mut p = use_state(|| Person {
            age: 16,
            families: vec![Person {
                age: 12,
                families: vec![],
            }],
        });

        let f = |p: &mut State<Person>| {
            assert_eq!(p.value().age, 16);

            let cloned = p.clone();

            p.set(|mut person| {
                person.age = 99;
                person
            } );

            assert_eq!(cloned.value().age, 99);
        };

        f(&mut p);

        // println!("{:#?}", p);
    }
}
