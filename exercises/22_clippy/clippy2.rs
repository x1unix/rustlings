fn main() {
    let mut res: i32 = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    while let Some(x) = option {
        match res.checked_add(x) {
            Some(v) => res = v,
            None => break,
        }
    }

    println!("{res}");
}
