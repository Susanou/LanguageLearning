use buffers::run;

fn main() {
    pollster::block_on(run());
}
