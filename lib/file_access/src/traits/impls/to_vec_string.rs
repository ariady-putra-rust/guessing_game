use crate::{traits::to_vec_string::*, *};

impl<Line: AsRef<str>> ToVecString<Line> for Vec<Line> {
    fn to_vec_string(&self) -> Vec<Text> {
        self.iter().map(|line| line.as_ref().to_string()).collect()
    }
}
