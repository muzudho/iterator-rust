use crate::{Sequence, SequenceIter};

/// 普段よく見る、 `for item in &items {` のような書き方をするためのトレイトです。  
impl IntoIterator for Sequence {
    type Item = Box<char>;

    /// `Self::IntoIter` がこれ。  
    type IntoIter = SequenceIter;

    /// イントゥ・イテレーターを返します。
    fn into_iter(self) -> Self::IntoIter {
        // イテレーターを返すのと同じ振る舞いで実装します。
        self.iter()
    }
}
