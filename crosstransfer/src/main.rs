use env_logger::{Builder, Env, Target};

fn main() {
    println!("Hello, world!");
    let env = Env::default().default_filter_or("debug");
    Builder::from_env(env).target(Target::Stdout).init();
    println!("Goodbye, world!");
}
