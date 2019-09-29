use std::marker::PhantomData;

struct Cacher<T, U, R>
    where
        U: Copy,
        R: Copy,
        T: Fn(U) -> R
{
    calculation: T,
    value: Option<R>,
    __phantom: PhantomData<U>,
}

impl<T, U, R> Cacher<T, U, R>
    where
        U: Copy,
        R: Copy,
        T: Fn(U) -> R
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
            __phantom: PhantomData,
        }
    }

    fn value(&mut self, arg: U) -> R {
        let current = self.value;
        match current {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}