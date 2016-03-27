const PI: f32 = 3.141592653589793;

pub trait Trig {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
}

impl Trig for f32 {
    fn sin(mut self) -> f32 {
        fn sin_imp(x: f32) -> f32 {
            x - x * x * x / 6.0 + x * x * x * x * x / 120.0 - x * x * x * x * x * x * x / 5040.0 // + x * x * x * x * x * x * x * x * x / 362880.0
        }

        self = self % (2.0 * PI);

        if self.is_sign_negative() {
            -(-self).sin()
        } else if self < PI / 2.0 {
            sin_imp(self)
        } else if self < PI {
            1.0 - sin_imp(self -  PI / 2.0)
        } else if self < 3.0 / 2.0 * PI {
            -sin_imp(self - PI)
        } else {
            sin_imp(self - 3.0 / 2.0 * PI) + 1.0
        }
    }

    fn cos(self) -> f32 {
        (self + PI / 2.0).sin()
    }

    fn tan(self) -> f32 {
        self.sin() / self.cos()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trig() {
        for i in -1000000..1000000 {
            let x = i as f32 / 7000.0;

            let dsin = x.sin() - Trig::sin(x);
            let dcos = x.cos() - Trig::cos(x);
            let dtan = x.tan() - Trig::tan(x);

            assert!(dsin.abs() < 0.22, "dsin = {}, x = {}, i = {}", dsin, x, i);
            assert!(dcos.abs() < 0.22, "dcos = {}, x = {}, i = {}", dcos, x, i);
            assert!(dtan.abs() < 0.22, "dtan = {}, x = {}, i = {}", dtan, x, i);
        }
    }
}
