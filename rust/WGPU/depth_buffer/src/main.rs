use depth_buffer::run;

fn main() {
    pollster::block_on(run());
}
