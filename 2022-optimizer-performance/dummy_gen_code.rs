pub const LEN_INPUTS: usize = 0;
pub const LEN_GRADIENT: usize = LEN_INPUTS + 1;
pub const LEN_OBJ_ENGS: usize = 0;
pub const LEN_CONSTR_ENGS: usize = 0;
pub const LEN_SECONDARY: usize = LEN_OBJ_ENGS + LEN_CONSTR_ENGS;

pub fn f(inputs: &[f64], gradient: &mut [f64], secondary: &mut [f64]) -> f64 {
    assert_eq!(inputs.len(), LEN_GRADIENT);
    assert_eq!(gradient.len(), LEN_GRADIENT);
    assert_eq!(secondary.len(), LEN_SECONDARY);
    0.0
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum InputMeta {
    Sampler,
    Pending,
}

pub const INPUTS: [InputMeta; LEN_INPUTS] = [];

pub const VARYING_VALUES: [f64; LEN_INPUTS] = [];
