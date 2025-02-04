fn main() {
    assert_eq!(range(&vec![1, 4, 3, 2]), vec![(1, 4)]);
    assert_eq!(range(&vec![1, 4, 5, 2, 3, 9, 8, 11, 0]), vec![(0, 5), (8, 9), (11, 11)]);
}

fn range(input: &[u32]) -> Vec<(u32, u32)> {
    let mut vec = input.to_vec();
    vec.sort();
    let mut vec = vec.into_iter().peekable();

    let mut result = Vec::new();
    let Some(mut prev) = vec.next() else {
        return result;
    };

    let mut start = prev;
    while let Some(curr) = vec.next() {
        if curr - prev > 1 {
            result.push((start, prev));
            start = curr;
        }
        prev = curr;
    }
    result.push((start, prev));

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{self, *};

    #[template]
    #[rstest]
    #[case::empty(vec![], vec![])]
    #[case::single_number(vec![8], vec![(8, 8)])]
    #[case::short_list(vec![1, 4, 3, 2], vec![(1, 4)])]
    #[case::long_list(vec![1, 4, 5, 2, 3, 9, 8, 11, 0], vec![(0, 5), (8, 9), (11, 11)])]
    #[case::long_list_with_single_number_in_the_middle(vec![1, 5, 2, 3, 9, 8, 0], vec![(0, 3), (5, 5), (8, 9)])]
    fn base(
        #[case] input: Vec<u32>,
        #[case] expected: Vec<(u32, u32)>,
    ) {}

    #[apply(base)]
    fn test_range(
        #[case] input: Vec<u32>,
        #[case] expected: Vec<(u32, u32)>,
    ) {
        assert_eq!(range(&input), expected);
    }
}
