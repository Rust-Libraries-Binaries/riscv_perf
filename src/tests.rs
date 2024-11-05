#[cfg(test)]
mod tests {
    use super::profiler::*;
    use super::visualizer::*;

    #[test]
    fn test_profiler() {
        let mut profiler = Profiler::new();
        let program = vec![0x00, 0x00, 0x00, 0x33]; // Example RISC-V binary
        profiler.load_program(&program);
        profiler.run();
        let profile_data = profiler.get_profile_data();
        assert!(profile_data.instructions_executed > 0);
    }

    #[test]
    fn test_visualizer() {
        let data = ProfileData {
            instructions_executed: 10,
            // Initialize other fields
        };
        visualize_profile_data(&data);
        // Add assertions to validate visualization
    }
}
