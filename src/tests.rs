use super::{range, Range};
#[test]
fn start_from_zero_range_test() {
    assert_eq!(
        range!(,,100).collect::<Vec<i32>>(),
        (0..100).collect::<Vec<i32>>()
    );
    assert_eq!(range!(,,7).collect::<Vec<i32>>(), vec![0, 1, 2, 3, 4, 5, 6]);
}
#[test]
fn infinite_range_test() {
    assert_eq!(
        range!(1i32,,).take(10).collect::<Vec<i32>>(),
        (1i32..).take(10).collect::<Vec<i32>>()
    );
    assert_eq!(
        range!(10i32,,).take(10).collect::<Vec<i32>>(),
        (10i32..).take(10).collect::<Vec<i32>>()
    );
}

#[test]
fn regular_range_test() {}

#[test]
fn step_range_test() {
    assert_eq!(
        range!(1,,3,,=10).collect::<Vec<i32>>(),
        (1i32..=10).step_by(3).collect::<Vec<i32>>()
    );
    assert_eq!(
        range!(2,,2,,=10).collect::<Vec<i32>>(),
        (2i32..=10).step_by(2).collect::<Vec<i32>>()
    );
    assert_eq!(
        range!(4,,4,,=10).collect::<Vec<i32>>(),
        (4..=10).step_by(4).collect::<Vec<i32>>()
    );
    assert_eq!(
        range!(-1,,-2,,=-10).collect::<Vec<i32>>(),
        vec![-1, -3, -5, -7, -9]
    );
}

#[test]
fn inclusive_from_zero_range_test() {
    assert_eq!(range!(,,4,,=10).collect::<Vec<i32>>(), vec![0, 4, 8],);
    assert_eq!(range!(,,-4,,=-10).collect::<Vec<i32>>(), vec![0, -4, -8],);
    assert_eq!(
        range!(,,2,,=10).collect::<Vec<i32>>(),
        vec![0, 2, 4, 6, 8, 10]
    );
    assert_eq!(range!(,,3,,=10).collect::<Vec<i32>>(), vec![0, 3, 6, 9]);
    assert_eq!(
        range!(,,-2,,=-10).collect::<Vec<i32>>(),
        vec![0, -2, -4, -6, -8, -10]
    );
}

#[test]
fn inclusive_range_test() {
    assert_eq!(range!(1,,6,,=20).collect::<Vec<i32>>(), vec![1, 7, 13, 19]);
    assert_eq!(range!(0,,10,,=10).collect::<Vec<i32>>(), vec![0, 10]);
    assert_eq!(
        range!(1,,1,,=6).collect::<Vec<i32>>(),
        vec![1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn range_str_test() {
    assert_eq!(
        range!("1..1..=6").collect::<Vec<i32>>(),
        vec![1, 2, 3, 4, 5, 6]
    );
    assert_eq!(range!("1..6..=20").collect::<Vec<i32>>(), vec![1, 7, 13, 19]);
    assert_eq!(range!("0..10..=10").collect::<Vec<i32>>(), vec![0, 10]);
    assert_eq!(range!(",,4,,=10").collect::<Vec<i32>>(), vec![0, 4, 8],);
    assert_eq!(range!(",,-4,,=-10").collect::<Vec<i32>>(), vec![0, -4, -8],);
    assert_eq!(
        range!(",,2,,=10").collect::<Vec<i32>>(),
        vec![0, 2, 4, 6, 8, 10]
    );
    assert_eq!(range!("..3..=10").collect::<Vec<i32>>(), vec![0, 3, 6, 9]);
    assert_eq!(
        range!(",,-2,,=-10").collect::<Vec<i32>>(),
        vec![0, -2, -4, -6, -8, -10]
    );
}
