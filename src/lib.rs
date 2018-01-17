use std::cmp;
use std::collections::binary_heap;
use std::iter;

// we have to make our own version of Peekable that doesn't demand mutability
struct QueueElement<I>
    where I: Iterator,
          I::Item: Ord,
{
    next: I::Item,
    rest: I,
}

impl<I> QueueElement<I>
    where I: Iterator,
          I::Item: Ord,
{
    fn iterate(self: Self) -> (I::Item, Option<Self>) {
        let QueueElement { next, mut rest } = self;
        let new_next = rest.next();
        let new_self = new_next.map(|next| QueueElement { next, rest });
        (next, new_self)
    }

    fn try_new(mut rest: I) -> Option<Self> {
        let try_next = rest.next();
        try_next.map(|next| QueueElement { next, rest })
    }
}

impl<I> PartialEq for QueueElement<I>
    where I: Iterator,
          I::Item: Ord,
{
    fn eq(self: &Self, other: &Self) -> bool {
        self.next == other.next
    }
}

impl<I> Eq for QueueElement<I>
    where I: Iterator,
          I::Item: Ord,
{
}

impl<I> PartialOrd for QueueElement<I>
    where I: Iterator,
          I::Item: Ord,
{
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&self.next, &other.next)
    }
}

impl<I> Ord for QueueElement<I>
    where I: Iterator,
          I::Item: Ord,
{
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering {
        Ord::cmp(&self.next, &other.next)
    }
}

pub struct IterQueue<I>
    where I: Iterator,
          I::Item: Ord,
{
    iters: binary_heap::BinaryHeap<QueueElement<I>>
}

impl<I> IterQueue<I>
    where I: Iterator,
          I::Item: Ord,
{
    pub fn new() -> Self {
        let iters = binary_heap::BinaryHeap::new();
        IterQueue { iters }
    }
}

impl<I> Iterator for IterQueue<I>
    where I: Iterator,
          I::Item: Ord,
{
    type Item = I::Item;
    fn next(self: &mut Self) -> Option<Self::Item> {
        while let Some(top) = self.iters.pop() {
            let (next, maybe_top) = top.iterate();
            maybe_top.map(|top| self.iters.push(top));
            return Some(next);
        }
        None
    }
}

impl<I> iter::FromIterator<I> for IterQueue<I>
    where I: Iterator,
          I::Item: Ord,
{
    fn from_iter<T>(raw_iters: T) -> Self
        where T: IntoIterator<Item = I>
    {
        let iters = raw_iters
            .into_iter()
            .flat_map(QueueElement::try_new)
            .collect();
        IterQueue { iters }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let nums = vec![
            vec![7, 5, 2],
            vec![6, 3, 1],
            vec![9, 4],
            vec![8],
        ];
        let iq: super::IterQueue<_> = nums
            .into_iter()
            .map(IntoIterator::into_iter)
            .collect();
        let flat_nums: Vec<_> = iq.collect();
        assert_eq!(flat_nums, vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}

