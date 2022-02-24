#![cfg_attr(feature = "std", feature(once_cell))]

trait OnceCellExt<T> {
    fn set_or<E>(&mut self, value: T, error: E) -> Result<(), E>;
    fn set_or_else<E, F>(&mut self, value: T, error: F) -> Result<(), E>
    where
        F: FnOnce() -> E;
    fn is_some(&self) -> bool;
    fn is_none(&self) -> bool;
}

macro_rules! once_cell_ext_impl {
    ($($ty:ty),+) => {
        $(impl<T> $crate::OnceCellExt<T> for $ty {
            fn set_or<E>(&mut self, value: T, error: E) -> Result<(), E> {
                self.set(value).map_err(|_| error)
            }

            fn set_or_else<E, F>(&mut self, value: T, error: F) -> Result<(), E>
            where
                F: FnOnce() -> E,
            {
                self.set(value).map_err(|_| error())
            }

            fn is_some(&self) -> bool {
                self.get().is_some()
            }

            fn is_none(&self) -> bool {
                self.get().is_none()
            }
        })*
    };
}

#[cfg(feature = "std")]
once_cell_ext_impl! {
    std::lazy::OnceCell<T>, std::lazy::SyncOnceCell<T>
}

once_cell_ext_impl! {
    once_cell::unsync::OnceCell<T>, once_cell::sync::OnceCell<T>
}
