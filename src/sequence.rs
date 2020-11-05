use crate::{Sequence, SequenceIter};
use std::fmt;

impl Sequence {
    /// イテレーター・インスタンスを生成します。  
    pub fn iter(self) -> SequenceIter {
        // イテレーションする都度、Iterインスタンスを作成します。
        SequenceIter {
            owner: Box::new(self),
            // カレントを設定します。
            curr: 0,
        }
    }
}

/// デバッグ出力。  
impl fmt::Debug for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        // ふつうはこう → for chr in &self.sequence {
        // .clone() して重そう。 `for chr in self {` と書けないの？
        for chr in (*self).clone() {
            buf.push_str(&format!("{:?}", chr));
        }
        write!(f, "{}", buf)
    }
}
