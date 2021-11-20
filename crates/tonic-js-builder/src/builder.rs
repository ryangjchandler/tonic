use crate::Var;

type BuilderCallbackFunction<T> = fn (&mut T);

#[derive(Debug)]
pub struct Builder {
    source: String,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            source: String::default(),
        }
    }

    pub fn var(&mut self, builder: BuilderCallbackFunction<Var>) -> &mut Self {
        let mut var = Var::new();

        builder(&mut var);

        self
    }
}

