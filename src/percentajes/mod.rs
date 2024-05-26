use crate::fractions::Fraction;

mod tests;

pub fn is_proportion(f1: Fraction, f2: Fraction) -> bool {
    if f1.has_x() || f2.has_x() {
        return false
    }
    (f1.get_a().unwrap() * f2.get_b().unwrap()) == (f1.get_b().unwrap() * f2.get_a().unwrap())
}
pub fn calc_proportion_value(f1: Fraction, f2: Fraction) -> f64 {
    // si dos tienen incognita no es calculable
    if f1.has_x() && f2.has_x() { return 0.0 }
    // si dos no tienen incognita ya esta resuelto
    if !f1.has_x() && !f2.has_x() { return 0.0 }

    if f1.get_a().is_some() && f2.get_b().is_some() {
        let aux_m = f1.get_a().unwrap() * f2.get_b().unwrap();
        return if f1.get_b().is_none() {
            aux_m / f2.get_a().unwrap()
        } else {
            aux_m / f1.get_a().unwrap()
        }
    }
    if f2.get_a().is_some() && f1.get_b().is_some() {
        let aux_m = f2.get_a().unwrap() * f1.get_b().unwrap();
        return if f1.get_a().is_none() {
            aux_m / f2.get_b().unwrap()
        } else {
            aux_m / f1.get_a().unwrap()
        }
    }
    0.0
}
pub fn is_directly_proportional(fractions: Vec<Fraction>) -> bool {
    let mut d: bool = false;
    for i in 0..fractions.len() {
        if (fractions.len()-1 < i) || (fractions.len()-1 < i+1) {
             return d;
        }
        let f1 = &fractions[i];
        let f2 = &fractions[i+1];
        if f1.calculate().unwrap() != f2.calculate().unwrap() { return false }
        d = true;
    }
    true
}
pub fn percentage_in_decimal(percentage: u16) -> f32 {
    (percentage / 100) as f32
}
pub fn percentage_of(percentage: u16, num: u16) -> f64 {
    (percentage * num / 100) as f64
}
pub fn calc_part(percentage: u16, total: u16) -> u32 {
    (total * percentage / 100) as u32
}
pub fn calc_percentage(total: u16, part: u16) -> u32 {
    (part * 100 / total) as u32
}
pub fn calc_total(part: u16, percentage: u16) -> u32 {
    (part * 100 / percentage) as u32
}