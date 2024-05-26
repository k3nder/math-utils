use crate::fractions::Fraction;

#[test]
fn calculate() {
    let f = Fraction::new(Some(6.0), Some(2.0));
    assert_eq!(f.calculate().unwrap(), 3.0);
}
#[test]
fn get_a() {
    let f = Fraction::new(Some(5.0), Some(0.0));
    assert_eq!(f.get_a().unwrap(), 5.0);
}
#[test]
fn get_b() {
    let f = Fraction::new(Some(0.0), Some(5.0));
    assert_eq!(f.get_b().unwrap(), 5.0);
}
#[test]
fn x_in_fraction_calc() {
    let f = Fraction::new(None, Some(5.0));
    assert_eq!(f.calculate(), None)
}