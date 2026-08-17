// RMSNorm Implementation - Root Mean Square Layer Normalization
// Reference: https://arxiv.org/abs/1910.07410

pub fn rmsnorm(input: &[f32], weight: &[f32], eps: f32) -> Vec<f32> {
    let len = input.len().min(weight.len());
    
    let ss: f32 = input[..len].iter().map(|&x| x * x).sum();
    let rms = (ss / len as f32 + eps).sqrt().recip();
    
    input[..len].iter()
        .zip(weight[..len].iter())
        .map(|(&x, &w)| x * w * rms)
        .collect()
}

pub fn rmsnorm_inplace(input: &mut [f32], weight: &[f32], eps: f32) {
    let len = input.len().min(weight.len());
    
    let ss: f32 = input[..len].iter().map(|&x| x * x).sum();
    let rms = (ss / len as f32 + eps).sqrt().recip();
    
    for i in 0..len {
        input[i] = input[i] * weight[i] * rms;
    }
}