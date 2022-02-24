use once_cell::{sync, unsync};

trait OnceCellExt<T> {
    fn set_or<E>(&mut self, value: T, error: E) -> Result<(), E>;
    fn set_or_else<E, F>(&mut self, value: T, error: F) -> Result<(), E>
    where
        F: FnOnce() -> E;
}

impl<T> OnceCellExt<T> for sync::OnceCell<T> {
    fn set_or<E>(&mut self, value: T, error: E) -> Result<(), E> {
        self.set(value).map_err(|_| error)
    }

    fn set_or_else<E, F>(&mut self, value: T, error: F) -> Result<(), E>
    where
        F: FnOnce() -> E,
    {
        self.set(value).map_err(|_| error())
    }
}

impl<T> OnceCellExt<T> for unsync::OnceCell<T> {
    fn set_or<E>(&mut self, value: T, error: E) -> Result<(), E> {
        self.set(value).map_err(|_| error)
    }

    fn set_or_else<E, F>(&mut self, value: T, error: F) -> Result<(), E>
    where
        F: FnOnce() -> E,
    {
        self.set(value).map_err(|_| error())
    }
}
