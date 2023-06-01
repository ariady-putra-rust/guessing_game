use crate::*;

pub trait ToVecString<T> {
    fn to_vec_string(&self) -> Vec<Text>;
}
