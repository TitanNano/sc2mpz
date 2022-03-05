use std::marker::PhantomData;

pub trait IndexedWrite<T> {
    fn write(&mut self, index: usize, value: T);
    fn field_len() -> usize;
}

pub struct IndexedWriter<'t, V, T>
where
    T: IndexedWrite<V>,
{
    inner: &'t mut T,
    index: usize,
    phantom: PhantomData<V>,
}

impl<'t, V, T> IndexedWriter<'t, V, T>
where
    T: IndexedWrite<V>,
{
    pub fn set_next(&mut self, value: V) {
        self.inner.write(self.index, value);
        self.index += 1;
    }

    pub fn new(target: &'t mut T) -> Self {
        Self {
            inner: target,
            index: 0,
            phantom: PhantomData,
        }
    }
}
