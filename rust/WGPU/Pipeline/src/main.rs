use Pipeline::run;

fn main() {
    pollster::block_on(run());
}
