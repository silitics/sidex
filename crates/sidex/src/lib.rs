#![doc = include_str!("../README.md")]

#[cfg(feature = "macros")]
pub use sidex_macros::*;
#[cfg(feature = "serde")]
pub use sidex_serde as serde;

// pub trait Mutate {
//     type Mutation;

//     fn mutate(&mut self, mutation: Self::Mutation);
// }

// pub enum Patch<T: Mutate> {
//     Set(Box<T>),
//     Mutate(Vec<T::Mutation>),
// }

// impl<T: Mutate> Patch<T> {
//     fn apply(self, value: &mut T) {
//         match self {
//             Patch::Set(new) => *value = *new,
//             Patch::Mutate(mutations) => {
//                 mutations
//                     .into_iter()
//                     .for_each(|mutation| value.mutate(mutation))
//             }
//         }
//     }
// }

// pub enum VecMutation<T: Mutate> {
//     Push(T),
//     Remove(usize),
//     MoveUp(usize),
//     MoveDown(usize),
//     Swap(usize, usize),
//     Mutate(usize, Patch<T>),
// }

// impl<T: Mutate> Mutate for Vec<T> {
//     type Mutation = VecMutation<T>;

//     fn mutate(&mut self, mutation: Self::Mutation) {
//         match mutation {
//             VecMutation::Push(element) => self.push(element),
//             VecMutation::Remove(index) => {
//                 if index < self.len() {
//                     self.remove(index);
//                 }
//             }
//             VecMutation::Swap(x, y) => {
//                 if x < self.len() && y < self.len() {
//                     self.swap(x, y);
//                 }
//             }
//             VecMutation::Mutate(index, mutation) => {
//                 if index < self.len() {
//                     mutation.apply(&mut self[index]);
//                 }
//             }
//             VecMutation::MoveUp(_) => todo!(),
//             VecMutation::MoveDown(_) => todo!(),
//         }
//     }
// }
