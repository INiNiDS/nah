use nah::too_many_arguments;

#[too_many_arguments("this should fail in deployment mode")]
fn blocked(a: i32, b: i32, c: i32, d: i32) -> i32 {
    a + b + c + d
}

fn main() {
    let _ = blocked(1, 2, 3, 4);
}

