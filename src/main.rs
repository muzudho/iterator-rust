pub mod sequence;
pub mod sequence_iter;

fn main() {
    println!("Start.");

    let seq = Sequence {
        sequence: vec![
            'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '!',
        ],
    };

    println!("{:?}", seq);
    // Hello, World!!
    assert_eq!(
        "'H''e''l''l''o'','' ''W''o''r''l''d''!''!'",
        format!("{:?}", seq)
    );

    println!("Finished.");
}

pub struct Sequence {
    sequence: Vec<char>,
}

/// 配列のインデックスを持ちます。
pub struct SequenceIter {
    owner: Box<Sequence>,
    curr: usize,
}
