use crate::Sequence;
use std::fmt;

/// 普段よく見るイテレーションのベースとなる、  
/// 普段あまり見ない `items.next()` を使ったイテレーションの実装です。  
impl Iterator for Sequence {
    // Self::Item ってこれ。
    type Item = Box<char>;

    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.sequence.len() {
            // .clone() するよりは Box でラッピングした方がいいだろうか？
            let item = Some(Box::new(self.sequence[self.curr]));
            self.curr += 1;
            return item;
        }

        return None;
    }
}

/*
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
*/

/// デバッグ出力。  
impl fmt::Debug for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        // .clone() して重そう。
        let mut copy = self.clone();
        loop {
            // `self.next() {` と書けないの？
            if let Some(chr) = copy.next() {
                buf.push_str(&format!("{:?}", chr));
            } else {
                break;
            }
        }
        /*
        // ふつうはこう → for chr in &self.sequence {
        // .clone() して重そう。 `for chr in self {` と書けないの？
        for chr in self {
            buf.push_str(&format!("{:?}", chr));
        }
        */
        write!(f, "{}", buf)
    }
}

/*
/// 普段よく見る、 `for item in &items {` のような書き方をするためのトレイトです。
impl IntoIterator for Sequence {
    type Item = Box<char>;

    /// `Self::IntoIter` がこれ。
    type IntoIter = Sequence;

    /// イントゥ・イテレーターを返します。
    fn into_iter(self) -> Self::IntoIter {
        // イテレーターを返すのと同じ振る舞いで実装します。
        self.iter()
    }
}
*/
