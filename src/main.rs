pub mod sequence;
pub mod sequence_into_iter;
pub mod sequence_iter;

fn main() {
    println!("Start.");

    let seq = Sequence {
        sequence: vec![
            'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '!',
        ],
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

/// イテレートされるもの。  
#[derive(Clone)]
pub struct Sequence {
    sequence: Vec<char>,
}

/// 自作のイテレーター。  
pub struct SequenceIter {
    owner: Box<Sequence>,
    /// 配列のインデックスのようなもの。
    curr: usize,
}
