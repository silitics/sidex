use thiserror::Error;

#[derive(Debug, Error)]
#[error("Unable to apply mutation: {0}.")]
pub struct Error(String);

impl Error {
    pub fn custom(message: String) -> Self {
        Self(message)
    }
}

pub trait Mutable {
    type Mutation;

    fn mutate(&mut self, mutation: Self::Mutation) -> Result<(), Error>;
}

pub enum Never {}

pub trait Immutable {}

impl<T: Immutable> Mutable for T {
    type Mutation = Never;

    fn mutate(&mut self, mutation: Self::Mutation) -> Result<(), Error> {
        match mutation {}
    }
}

pub enum NumericMutation<T: Mutable> {
    Add(T),
    Sub(T),
    Mul(T),
    Div(T),
    Set(T),
}

pub enum SequenceMutation<T: Mutable> {
    Push(T),
    Mutate(MutateElement<T>),
}

pub struct MutateElement<T: Mutable> {
    index: usize,
    mutation: T::Mutation,
}

impl<T: Mutable> Mutable for Vec<T> {
    type Mutation = SequenceMutation<T>;

    fn mutate(&mut self, mutation: SequenceMutation<T>) -> Result<(), Error> {
        match mutation {
            SequenceMutation::Push(element) => self.push(element),
            SequenceMutation::Mutate(MutateElement { index, mutation }) => {
                if let Some(element) = self.get_mut(index) {
                    return element.mutate(mutation);
                } else {
                    return Err(Error::custom("Index out of bounds.".to_owned()));
                }
            }
        }
        Ok(())
    }
}
