#[cfg(test)]
mod tests;

pub struct Fraction {
    a: Option<f64>,
    b: Option<f64>
}
impl Fraction {
    pub fn new(a: Option<f64>, b: Option<f64>) -> Self {
        Fraction {
            a, b
        }
    }
    pub fn calculate(&self) -> Option<f64> {
        if self.a.is_none() || self.b.is_none() { return None; }
        Some(self.a.unwrap() / self.b.unwrap())
    }
    pub fn get_a(&self) -> Option<f64> {
        self.a
    }
    pub fn get_b(&self) -> Option<f64> {
        self.b
    }
    pub fn has_x(&self) -> bool {
        ( self.a.is_none() || self.b.is_none() )
    }
}

