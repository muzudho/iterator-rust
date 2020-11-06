# lesson-of-rust-iterator

## Run

```shell
cargo run
# cargo run --release
```

## Output

```plain
Start: sub1.
[seq1]
[Sequence1.next] curr=0
[Sequence1.next] curr=1
[Sequence1.next] curr=2
[Sequence1.next] curr=3
[Sequence1.next] curr=4
[Sequence1.next] curr=5
[Sequence1.next] curr=6
[Sequence1.next] curr=7
[Sequence1.next] curr=8
[Sequence1.next] curr=9
[Sequence1.next] curr=10
[Sequence1.next] curr=11
[Sequence1.next] curr=12
[Sequence1.next] curr=13
[Sequence1.next] curr=14
'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
[&seq1]
[Sequence1.next] curr=0
[Sequence1.next] curr=1
[Sequence1.next] curr=2
[Sequence1.next] curr=3
[Sequence1.next] curr=4
[Sequence1.next] curr=5
[Sequence1.next] curr=6
[Sequence1.next] curr=7
[Sequence1.next] curr=8
[Sequence1.next] curr=9
[Sequence1.next] curr=10
[Sequence1.next] curr=11
[Sequence1.next] curr=12
[Sequence1.next] curr=13
[Sequence1.next] curr=14
'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
[Sequence1.next] curr=0
[Sequence1.next] curr=1
[Sequence1.next] curr=2
[Sequence1.next] curr=3
[Sequence1.next] curr=4
[Sequence1.next] curr=5
[Sequence1.next] curr=6
[Sequence1.next] curr=7
[Sequence1.next] curr=8
[Sequence1.next] curr=9
[Sequence1.next] curr=10
[Sequence1.next] curr=11
[Sequence1.next] curr=12
[Sequence1.next] curr=13
[Sequence1.next] curr=14
Finished: sub1.
Start: sub2.
[seq2]
[Sequence2.next] curr=0
[Sequence2.next] curr=1
[Sequence2.next] curr=2
[Sequence2.next] curr=3
[Sequence2.next] curr=4
[Sequence2.next] curr=5
[Sequence2.next] curr=6
[Sequence2.next] curr=7
[Sequence2.next] curr=8
[Sequence2.next] curr=9
[Sequence2.next] curr=10
[Sequence2.next] curr=11
[Sequence2.next] curr=12
[Sequence2.next] curr=13
[Sequence2.next] curr=14
'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
[&seq2]
[Sequence2.next] curr=0
[Sequence2.next] curr=1
[Sequence2.next] curr=2
[Sequence2.next] curr=3
[Sequence2.next] curr=4
[Sequence2.next] curr=5
[Sequence2.next] curr=6
[Sequence2.next] curr=7
[Sequence2.next] curr=8
[Sequence2.next] curr=9
[Sequence2.next] curr=10
[Sequence2.next] curr=11
[Sequence2.next] curr=12
[Sequence2.next] curr=13
[Sequence2.next] curr=14
'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
[Sequence2.next] curr=0
[Sequence2.next] curr=1
[Sequence2.next] curr=2
[Sequence2.next] curr=3
[Sequence2.next] curr=4
[Sequence2.next] curr=5
[Sequence2.next] curr=6
[Sequence2.next] curr=7
[Sequence2.next] curr=8
[Sequence2.next] curr=9
[Sequence2.next] curr=10
[Sequence2.next] curr=11
[Sequence2.next] curr=12
[Sequence2.next] curr=13
[Sequence2.next] curr=14
Finished: sub2.
Start: sub3.
[seq3]
[Sequence3.cursor] New Sequence3Cursor.
[Sequence3Cursor.next] curr=0
[Sequence3Cursor.next] curr=1
[Sequence3Cursor.next] curr=2
[Sequence3Cursor.next] curr=3
[Sequence3Cursor.next] curr=4
[Sequence3Cursor.next] curr=5
[Sequence3Cursor.next] curr=6
[Sequence3Cursor.next] curr=7
[Sequence3Cursor.next] curr=8
[Sequence3Cursor.next] curr=9
[Sequence3Cursor.next] curr=10
[Sequence3Cursor.next] curr=11
[Sequence3Cursor.next] curr=12
[Sequence3Cursor.next] curr=13
[Sequence3Cursor.next] curr=14
'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
[&seq3]
[Sequence3.cursor] New Sequence3Cursor.
[Sequence3Cursor.next] curr=0
[Sequence3Cursor.next] curr=1
[Sequence3Cursor.next] curr=2
[Sequence3Cursor.next] curr=3
[Sequence3Cursor.next] curr=4
[Sequence3Cursor.next] curr=5
[Sequence3Cursor.next] curr=6
[Sequence3Cursor.next] curr=7
[Sequence3Cursor.next] curr=8
[Sequence3Cursor.next] curr=9
[Sequence3Cursor.next] curr=10
[Sequence3Cursor.next] curr=11
[Sequence3Cursor.next] curr=12
[Sequence3Cursor.next] curr=13
[Sequence3Cursor.next] curr=14
'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
[Sequence3.cursor] New Sequence3Cursor.
[Sequence3Cursor.next] curr=0
[Sequence3Cursor.next] curr=1
[Sequence3Cursor.next] curr=2
[Sequence3Cursor.next] curr=3
[Sequence3Cursor.next] curr=4
[Sequence3Cursor.next] curr=5
[Sequence3Cursor.next] curr=6
[Sequence3Cursor.next] curr=7
[Sequence3Cursor.next] curr=8
[Sequence3Cursor.next] curr=9
[Sequence3Cursor.next] curr=10
[Sequence3Cursor.next] curr=11
[Sequence3Cursor.next] curr=12
[Sequence3Cursor.next] curr=13
[Sequence3Cursor.next] curr=14
Finished: sub3.
Start: sub4.
[seq4]
[Sequence4.into_iter] New Sequence4IntoIter.
[Sequence4IntoIter.next] curr_a=0 curr_b=0
[Sequence4IntoIter.next] curr_a=0 curr_b=1
[Sequence4IntoIter.next] curr_a=0 curr_b=2
[Sequence4IntoIter.next] curr_a=0 curr_b=3
[Sequence4IntoIter.next] curr_a=0 curr_b=4
[Sequence4IntoIter.next] curr_a=0 curr_b=5
[Sequence4IntoIter.next] curr_a=0 curr_b=6
[Sequence4IntoIter.next] curr_a=0 curr_b=7
[Sequence4IntoIter.next] curr_a=0 curr_b=8
[Sequence4IntoIter.next] curr_a=0 curr_b=9
[Sequence4IntoIter.next] curr_a=0 curr_b=10
[Sequence4IntoIter.next] curr_a=0 curr_b=11
[Sequence4IntoIter.next] curr_a=0 curr_b=12
[Sequence4IntoIter.next] curr_a=0 curr_b=13
[Sequence4IntoIter.next] curr_a=0 curr_b=14
'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
[&seq4]
[Sequence4.into_iter] New Sequence4IntoIter.
[Sequence4IntoIter.next] curr_a=0 curr_b=0
[Sequence4IntoIter.next] curr_a=0 curr_b=1
[Sequence4IntoIter.next] curr_a=0 curr_b=2
[Sequence4IntoIter.next] curr_a=0 curr_b=3
[Sequence4IntoIter.next] curr_a=0 curr_b=4
[Sequence4IntoIter.next] curr_a=0 curr_b=5
[Sequence4IntoIter.next] curr_a=0 curr_b=6
[Sequence4IntoIter.next] curr_a=0 curr_b=7
[Sequence4IntoIter.next] curr_a=0 curr_b=8
[Sequence4IntoIter.next] curr_a=0 curr_b=9
[Sequence4IntoIter.next] curr_a=0 curr_b=10
[Sequence4IntoIter.next] curr_a=0 curr_b=11
[Sequence4IntoIter.next] curr_a=0 curr_b=12
[Sequence4IntoIter.next] curr_a=0 curr_b=13
[Sequence4IntoIter.next] curr_a=0 curr_b=14
'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
[Sequence4.into_iter] New Sequence4IntoIter.
[Sequence4IntoIter.next] curr_a=0 curr_b=0
[Sequence4IntoIter.next] curr_a=0 curr_b=1
[Sequence4IntoIter.next] curr_a=0 curr_b=2
[Sequence4IntoIter.next] curr_a=0 curr_b=3
[Sequence4IntoIter.next] curr_a=0 curr_b=4
[Sequence4IntoIter.next] curr_a=0 curr_b=5
[Sequence4IntoIter.next] curr_a=0 curr_b=6
[Sequence4IntoIter.next] curr_a=0 curr_b=7
[Sequence4IntoIter.next] curr_a=0 curr_b=8
[Sequence4IntoIter.next] curr_a=0 curr_b=9
[Sequence4IntoIter.next] curr_a=0 curr_b=10
[Sequence4IntoIter.next] curr_a=0 curr_b=11
[Sequence4IntoIter.next] curr_a=0 curr_b=12
[Sequence4IntoIter.next] curr_a=0 curr_b=13
[Sequence4IntoIter.next] curr_a=0 curr_b=14
Finished: sub4.
```
