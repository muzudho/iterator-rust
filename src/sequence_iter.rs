use crate::SequenceIter;

impl Iterator for SequenceIter {
    // Self::Item ってこれ。
    type Item = char;

    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.owner.sequence.len() {
            // TODO .clone() していて重そう。
            let item = Some(self.owner.sequence[self.curr].clone());
            self.curr += 1;
            return item;
        }

        return None;
    }
}
