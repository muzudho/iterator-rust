pub mod sequence1;
pub mod sequence2;

use crate::sequence1::Sequence1;
use crate::sequence2::Sequence2;

fn main() {
    sub1();
    sub2();
}

fn sub1() {
    println!("Start: sub1.");
    let seq = Sequence1 {
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
    println!("Finished: sub1.");
}

fn sub2() {
    println!("Start: sub2.");
    let seq = Sequence2 {
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
    println!("Finished: sub2.");
}
