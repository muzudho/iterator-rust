use std::fmt;
// use std::iter;

/// イテレートできるもの。  
#[derive(Clone)]
pub struct Sequence4 {
    /// String型は Unicodeなのでイテレートが案外難しい。 `Vec<char>` にしたのが工夫。  
    pub sequence: Vec<char>,
    /// イテレートで使う配列のインデックスのようなもの。  
    /// しかし `&mut self` でないと イテレートできない(`iter_mut()相当`)よな。  
    pub curr: usize,
}

/// 普段よく見る `for item in &items {` イテレーションの下地となる、  
/// 普段あまり表に出てこない `items.next()` を使ったイテレーションの実装です。  
impl Iterator for Sequence4 {
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

/// デバッグ出力。  
impl fmt::Debug for Sequence4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        // イテレートするたびに .clone() するのは重くないか？
        for chr in self.clone() {
            buf.push_str(&format!("{:?}", chr));
        }
        write!(f, "{}", buf)
    }
}

/*
/// 普段よく見る、 `for item in &items {` のような書き方をするためのトレイトです。
impl IntoIterator for Sequence4 {
    type Item = Box<char>;

    /// `Self::IntoIter` がこれ。
    type IntoIter = Sequence4IntoIter;

    /// イントゥ・イテレーターを返します。
    fn into_iter(self) -> Self::IntoIter {
        // イテレーションする都度、インスタンスを作成します。
        Sequence4IntoIter {
            owner: self,
            // カレントを設定します。
            curr: 0,
        }
    }
}
*/
impl<'a> IntoIterator for &'a Sequence4 {
    type Item = Box<char>;
    type IntoIter = Sequence4IntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Sequence4IntoIter {
            owner: self,
            // カレントを設定します。
            curr: 0,
        }
    }
}

/*
struct Sequence4IntoIter {
    owner: Sequence4,
    curr: usize,
}
*/
pub struct Sequence4IntoIter<'a> {
    owner: &'a Sequence4,
    curr: usize,
}

/*
/// 普段よく見る `for item in &items {` イテレーションの下地となる、
/// 普段あまり表に出てこない `items.next()` を使ったイテレーションの実装です。
impl Iterator for Sequence4IntoIter {
    // Self::Item ってこれ。
    type Item = Box<char>;

    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.owner.sequence.len() {
            // .clone() するよりは Box でラッピングした方がいいだろうか？
            let item = Some(Box::new(self.owner.sequence[self.curr]));
            self.curr += 1;
            return item;
        }

        return None;
    }
}
*/
impl<'a> Iterator for Sequence4IntoIter<'a> {
    type Item = Box<char>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.owner.sequence.len() {
            // .clone() するよりは Box でラッピングした方がいいだろうか？
            let item = Some(Box::new(self.owner.sequence[self.curr]));
            self.curr += 1;
            return item;
        }

        return None;
    }
}
