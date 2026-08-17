// SiLU Implementation - Sigmoid Linear Unit (Swish)
// f(x) = x * sigmoid(x)
// Reference: https://arxiv.org/abs/1612.08053

pub fn silu(x: &[f32]) -> Vec<f32> {
    x.iter().map(|&v| v / (1.0 + (-v).exp())).collect()
}

pub fn silu_inplace(x: &mut [f32]) {
    for v in x.iter_mut() {
        *v = *v / (1.0 + (-*v).exp());
    }
}