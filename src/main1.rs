mod arrays;
mod cli;
mod conditions;
mod enums;
mod functions;
mod loops;
mod pointer_refs;
mod structs;
mod testing;
mod variables;
mod vectors;
fn main() {
    variables::run();
    arrays::go();
    testing::run();
    vectors::run();
    conditions::run();
    loops::run();
    functions::run();
    pointer_refs::run();
    structs::run();
    enums::run();
    cli::run();
}
