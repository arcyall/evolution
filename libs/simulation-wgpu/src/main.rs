fn main() {
    pollster::block_on(lib_simulation_wgpu::run());
}
