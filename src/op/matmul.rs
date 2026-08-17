
// Matrix Multiplication Implementation
// Naive but correct GEMM for inference

pub fn matmul(a: &[f32], b: &[f32], m: usize, n: usize, k: usize, output: &mut [f32]) {
    output.fill(0.0);
    
    for i in 0..m {
        for j in 0..n {
            let mut sum = 0.0;
            for l in 0..k {
                sum += a[i * k + l] * b[l * n + j];
            }
            output[i * n + j] = sum;
        }
    }
}

pub fn matmul_transpose(a: &[f32], b: &[f32], m: usize, n: usize, k: usize, output: &mut [f32]) {
    output.fill(0.0);
    
    for i in 0..m {
        for j in 0..k {
            let mut sum = 0.0;
            for l in 0..n {
                sum += a[i * n + l] * b[j * n + l];
            }
            output[i * k + j] = sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_matmul_basic() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![1.0, 0.0, 0.0, 1.0];
        let mut output = vec![0.0; 4];
        
        matmul(&a, &b, 2, 2, 2, &mut output);
        assert_eq!(output, vec![1.0, 2.0, 3.0, 4.0]);
    }
}

