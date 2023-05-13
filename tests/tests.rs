// //! Integration tests with rstest. Also see rstest's listed case-enumeration.
// //! https://docs.rs/rstest/latest/rstest/#creating-a-test-for-each-combinations-of-given-values
// use arbitrary::Arbitrary;
// use quickcheck_macros::quickcheck;
// use rstest::{fixture, rstest};
// use test_log::test as ltest; // logs in tests, can wrap other test macros
// use log::info;
// // use tracing::info;

// /// a template of some context to take into functions
// #[derive(Clone, Debug, Eq, PartialEq, Arbitrary)]
// struct Workbench {
//   b: bool,
//   n: usize,
// }


// // set up test state in a workbench fixture.
// #[fixture]
// fn workbench() -> Workbench { Workbench { b: true, n: 0 } }

// #[rstest] // use it as context in tests, and to set-up case-by-case fuzzing
// #[case(0, true, true)]
// #[case(1, true, false)]
// #[ltest] // must follow rtest
// fn test_workbench(workbench: Workbench, #[case] n: usize, #[case] b: bool, #[case] out: bool) {
//   info::("test logged");
//   let wb = Workbench { n, b };
//   let matches = workbench == wb;
//   assert_eq!(matches, out);
// }

// // Baby fuzz: create 4 test cases checking n<m.
// #[rstest]
// fn test_enumerative(#[values(0, 4)] n: usize, #[values(7, 8)] m: usize) { assert!(n < m) }

// // small-fuzzing
// fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
//   let mut rev = vec![];
//   for x in xs.iter() {
//     rev.insert(0, x.clone())
//   }
//   rev
// }

// // fuzz, declare quickcheck on any argument implementing Arbitrary
// #[quickcheck]
// fn prop(xs: Vec<u32>) -> bool { xs == reverse(&reverse(&xs)) }
