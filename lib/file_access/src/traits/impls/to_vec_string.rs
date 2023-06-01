use crate::{traits::to_vec_string::*, *};

impl<T: AsRef<str>> ToVecString<T> for Vec<T> {
    fn to_vec_string(&self) -> Vec<Text> {
        self.iter().map(|line| line.as_ref().to_string()).collect()
    }
}
