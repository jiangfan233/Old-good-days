use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use crate::StateLib::persist::{Persist, PersistedOrigin};


#[derive(Debug)]
pub struct RefContainer<T>(pub Rc<RefCell<T>>);

impl<T> RefContainer<T> {

    pub fn new(v: T) -> Self {
        Self( Rc::new(RefCell::new(v)) )
    }

    pub fn current(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn current_mut(&mut self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }

    pub fn set_current(&mut self, t: T) {
        *self.current_mut() = t; 
    }

}

impl<T> Clone for RefContainer<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}


impl<T: 'static> Persist for RefContainer<T> {
    fn ptr(&self) -> PersistedOrigin {
        PersistedOrigin
    }
}