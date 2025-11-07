pub fn generate(parsed: i64) {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", parsed);
    println!("  ret");
}
