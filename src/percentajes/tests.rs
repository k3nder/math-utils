use crate::fractions::Fraction;
use crate::percentajes::{calc_part, calc_percentage, calc_proportion_value, calc_total, is_directly_proportional, is_proportion, percentage_in_decimal, percentage_of};

#[test]
fn calc() {
    assert!(is_proportion(Fraction::new(Some(1.0), Some(1.2)), Fraction::new(Some(2.5), Some(3.0))))
}
#[test]
fn calc_proportion() {
    let x = calc_proportion_value(Fraction::new(Some(2.1), Some(18.0)), Fraction::new(Some(3.5), None));
    assert_eq!(x, 30.0);
}
#[test]
fn is_directly_proportional_test() {
    assert!(is_directly_proportional(vec![
        Fraction::new(Some(1.0), Some(1.0)),
        Fraction::new(Some(2.0), Some(2.0)),
        Fraction::new(Some(3.0), Some(3.0))
    ]))
}
#[test]
fn not_directly_proportional_test() {
    assert!(!is_directly_proportional(vec![
        Fraction::new(Some(2.0), Some(2.3)),
        Fraction::new(Some(5.0), Some(435.0))
    ]))
}
#[test]
fn percentage_in_decimal_test() {
    let x = percentage_in_decimal(100);
    assert_eq!(x, 1.0);
}
#[test]
fn percentage_of_test() {
    let x = percentage_of(30, 10);
    assert_eq!(x, 3.0);
}
#[test]
fn calc_part_test() {
    let x = calc_part(10, 30);
    assert_eq!(x, 3);
}
#[test]
fn calc_percentage_test() {
    let x = calc_percentage(20, 10);
    assert_eq!(x, 50);
}
#[test]
fn calc_total_test() {
    let x = calc_total(10, 2);
    assert_eq!(x, 500);
}