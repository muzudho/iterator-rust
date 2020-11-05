pub mod sequence;

fn main() {
    println!("Start.");

    let seq = Sequence {
        sequence: vec![
            'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '!',
        ],
        curr: 0,
    };

    // println!("{:?}", &seq.next());

    // Hello, World!! の１文字ずつをシングルクォーテーションで囲んだもの。
    println!("{:?}", &seq);
    assert_eq!(
        "'H''e''l''l''o'','' ''W''o''r''l''d''!''!'",
        format!("{:?}", &seq)
    );

    println!("Finished.");
}

/// イテレートできるもの。  
#[derive(Clone)]
pub struct Sequence {
    sequence: Vec<char>,
    /// イテレートで使う配列のインデックスのようなもの。
    curr: usize,
}
