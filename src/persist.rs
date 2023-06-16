
pub struct PersistedOrigin;

pub trait Persist: Clone + 'static {
    fn ptr(&self) -> PersistedOrigin {
        PersistedOrigin
    }
}