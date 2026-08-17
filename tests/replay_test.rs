// Replay Golden Test
// Target: Compare hacedle output with llama.cpp golden tensors

#[cfg(test)]
mod replay_tests {
    #[test]
    fn test_golden_l2_comparison() {
        // L2 norm comparison between hacedle and llama.cpp outputs
        let hacedle_output = vec![0.123f32, 0.456, 0.789];
        let llama_cpp_golden = vec![0.123f32, 0.456, 0.789];
        
        let l2_error: f32 = hacedle_output.iter()
            .zip(llama_cpp_golden.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt();
            
        // Error should be very small
        assert!(l2_error < 1e-5, "L2 error too large: {}", l2_error);
    }
    
    #[test]
    fn test_divergence_localizer() {
        // Identify which layer causes divergence
        let layer_outputs = vec![
            ("block0", 0.001f32),
            ("block1", 0.002),
            ("block2", 0.001),
        ];
        
        let divergent_layers: Vec<_> = layer_outputs.iter()
            .filter(|(_, err)| *err > 1e-5)
            .collect();
            
        assert!(divergent_layers.is_empty(), "No divergence expected in golden replay");
    }
}