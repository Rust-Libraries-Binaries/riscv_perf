pub mod profiler;
pub mod visualizer;

pub fn profile_program(program: &[u8]) -> profiler::ProfileData {
    let mut profiler = profiler::Profiler::new();
    profiler.load_program(program);
    profiler.run();
    profiler.get_profile_data()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_program() {
        let program = vec![0x00, 0x00, 0x00, 0x33]; // Example RISC-V binary
        let profile_data = profile_program(&program);
        assert!(profile_data.instructions_executed > 0);
    }
}
