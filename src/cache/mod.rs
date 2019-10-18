



struct Cacher<T, R>
{
    calculation: T,
    value: Option<R>,
}

impl<T, R> Cacher<T, R> {
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
        }
    }


    fn value<U>(&mut self, arg: U) -> &R
        where T: FnMut(U) -> R
    {
        let current: &mut Option<R> = &mut self.value;
        match current {
            Some(ref v) => v,
            None => {
                let v: R = (self.calculation)(arg);
                *current = Some(v);
                current.as_ref().unwrap()
            }
        }
    }


    // https://github.com/rust-lang/rust/issues/53589#issuecomment-539593375
    fn value2<U>(&mut self, arg: U) -> &R
        where T: FnMut(U) -> R
    {
        if self.value.is_none() {
            self.value = Some((self.calculation)(arg));
        }

        self.value.as_ref().unwrap()
    }


    fn value3<U>(&mut self, arg: U) -> &R
        where T: FnMut(U) -> R
    {
        if self.is_none() {
            self.value = Some((self.calculation)(arg));
        }

        self.value.as_ref().unwrap()
    }

    fn is_some(&self) -> bool {
        match self.value {
            Some(_) => true,
            None => false,
        }
    }

    fn is_none(&self) -> bool {
        !self.is_some()
    }
}


#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[test]
    fn it_works() {
        let mut expensive_result = Cacher::new(|input: i32| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            input * 2
        });

        println!(
            "Today, do {} pushups!",
            expensive_result.value3(1)
        );

        println!(
            "Next, do {} situps!",
            expensive_result.value3(1)
        );

        assert_eq!(2, *expensive_result.value3(1));
    }
}