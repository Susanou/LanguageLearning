use pipeline::run;

fn main() {
    pollster::block_on(run());
}
