use std::fmt;

/// イテレートできるもの。  
#[derive(Clone)]
pub struct Sequence4 {
    /// String型は Unicodeなのでイテレートが案外難しい。 `Vec<char>` にしたのが工夫。  
    pub sequence: Vec<char>,
    /// 直(非参照)で イテレートするときのカーソル。  
    /// `&mut self` で イテレートするとき、これを使う。  
    pub curr_a: usize,
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
        println!("[Sequence4.next] curr_a={}", self.curr_a);
        if self.curr_a < self.sequence.len() {
            // .clone() するよりは Box でラッピングした方がいいだろうか？
            let item = Some(Box::new(self.sequence[self.curr_a]));
            self.curr_a += 1;
            return item;
        }

        return None;
    }
}

/// デバッグ出力。  
impl fmt::Debug for Sequence4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        // IntoIterator を実装したので、 `self.clone()` と書かずに済んだ。
        for chr in self {
            buf.push_str(&format!("{:?}", chr));
        }
        write!(f, "{}", buf)
    }
}

/// 普段よく見る、 `for item in &items {` のような書き方をするためのトレイトです。
impl<'a> IntoIterator for &'a Sequence4 {
    type Item = Box<char>;

    /// `Self::IntoIter` がこれ。
    type IntoIter = Sequence4IntoIter<'a>;

    /// イントゥ・イテレーターを返します。
    fn into_iter(self) -> Self::IntoIter {
        // イテレーションする都度、インスタンスを作成します。
        Sequence4IntoIter {
            owner: self,
            // カレントを設定します。
            curr_b: 0,
        }
    }
}

pub struct Sequence4IntoIter<'a> {
    owner: &'a Sequence4,
    /// `for item in &items {` のときにイテレートするときのカーソル。
    curr_b: usize,
}

/// 普段よく見る `for item in &items {` イテレーションの下地となる、
/// 普段あまり表に出てこない `items.next()` を使ったイテレーションの実装です。
impl<'a> Iterator for Sequence4IntoIter<'a> {
    // Self::Item ってこれ。
    type Item = Box<char>;
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Self::Item> {
        println!(
            "[Sequence4IntoIter.next] curr_a={} curr_b={}",
            self.owner.curr_a, self.curr_b
        );
        if self.curr_b < self.owner.sequence.len() {
            // .clone() するよりは Box でラッピングした方がいいだろうか？
            let item = Some(Box::new(self.owner.sequence[self.curr_b]));
            self.curr_b += 1;
            return item;
        }

        return None;
    }
}
