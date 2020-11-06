pub mod sequence1;
pub mod sequence2;
pub mod sequence3;
pub mod sequence4;

use crate::sequence1::Sequence1;
use crate::sequence2::Sequence2;
use crate::sequence3::Sequence3;
use crate::sequence4::Sequence4;

fn main() {
    sub1();
    sub2();
    sub3();
    sub4();

    /*
    let v = &vec![0, 1, 2];
    for i in v {
        println!("{} ", i);
    }
    for i in v {
        println!("{} ", i);
    }
    */
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
    println!(
        "[seq1]
{:?}
[&seq1]
{:?}",
        seq, &seq
    );
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

    // Hello, World!! の１文字ずつをシングルクォーテーションで囲んだもの。
    println!(
        "[seq2]
{:?}
[&seq2]
{:?}",
        seq, &seq
    );
    assert_eq!(
        "'H''e''l''l''o'','' ''W''o''r''l''d''!''!'",
        format!("{:?}", &seq)
    );
    println!("Finished: sub2.");
}

fn sub3() {
    println!("Start: sub3.");
    let seq = Sequence3 {
        sequence: vec![
            'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '!',
        ],
    };

    // Hello, World!! の１文字ずつをシングルクォーテーションで囲んだもの。
    println!(
        "[seq3]
{:?}
[&seq3]
{:?}",
        seq, &seq
    );
    assert_eq!(
        "'H''e''l''l''o'','' ''W''o''r''l''d''!''!'",
        format!("{:?}", &seq)
    );
    println!("Finished: sub3.");
}

fn sub4() {
    println!("Start: sub4.");
    let seq = Sequence4 {
        sequence: vec![
            'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '!',
        ],
        curr_a: 0,
    };

    // Hello, World!! の１文字ずつをシングルクォーテーションで囲んだもの。
    println!(
        "[seq4]
{:?}
[&seq4]
{:?}",
        seq, &seq
    );
    assert_eq!(
        "'H''e''l''l''o'','' ''W''o''r''l''d''!''!'",
        format!("{:?}", &seq)
    );
    println!("Finished: sub4.");
}
