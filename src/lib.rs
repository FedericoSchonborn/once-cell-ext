#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OnceOption<T>(Option<T>);

impl<T> OnceOption<T> {
    pub fn into_inner(self) -> Option<T> {
        self.0
    }

    pub fn as_ref(&self) -> Option<&T> {
        self.0.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut()
    }
}
