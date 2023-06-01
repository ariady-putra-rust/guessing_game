use crate::*;

pub trait ToVecString {
    fn to_vec_string(&self) -> Vec<Text>;
}
