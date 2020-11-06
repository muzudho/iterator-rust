//! Sequence と Iterator を別インスタンスとしても、  
//! Sequence の .clone() は必要。

use std::fmt;

/// イテレートできるもの。 Clone derive を外せない。  
#[derive(Clone)]
pub struct Sequence3 {
    /// String型は Unicodeなのでイテレートが案外難しい。 `Vec<char>` にしたのが工夫。  
    pub sequence: Vec<char>,
}

impl Sequence3 {
    pub fn cursor(&self) -> Sequence3Cursor {
        println!("[Sequence3.cursor] New Sequence3Cursor.");
        Sequence3Cursor {
            // ここで .clone() 要る。意味ない。イテレートするたびに .clone() するのは重くないか？
            owner: Box::new(self.clone()),
            curr: 0,
        }
    }
}

/// カーソルを分離。
pub struct Sequence3Cursor {
    pub owner: Box<Sequence3>,
    /// イテレートで使う配列のインデックスのようなもの。  
    /// しかし `&mut self` でないと イテレートできない(`iter_mut()相当`)よな。  
    pub curr: usize,
}

/// 普段よく見る `for item in &items {` イテレーションの下地となる、  
/// 普段あまり表に出てこない `items.next()` を使ったイテレーションの実装です。  
impl Iterator for Sequence3Cursor {
    // Self::Item ってこれ。
    type Item = Box<char>;

    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Self::Item> {
        println!("[Sequence3Cursor.next] curr={}", self.curr);
        if self.curr < self.owner.sequence.len() {
            // .clone() するよりは Box でラッピングした方がいいだろうか？
            let item = Some(Box::new(self.owner.sequence[self.curr]));
            self.curr += 1;
            return item;
        }

        return None;
    }
}

/// デバッグ出力。  
impl fmt::Debug for Sequence3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for chr in self.cursor() {
            buf.push_str(&format!("{:?}", chr));
        }
        write!(f, "{}", buf)
    }
}
