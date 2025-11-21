use crate::node::Node;

pub fn generate_node(node: &Node) {
    match node {
        Node::Number { value} => {
            println!("  push {}", value);
        }
        Node::Add { lhs, rhs } => {
            generate_node(lhs);
            generate_node(rhs);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  add rax, rdi");
            println!("  push rax");
        }
        Node::Sub { lhs, rhs } => {
            generate_node(lhs);
            generate_node(rhs);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  sub rax, rdi");
            println!("  push rax");
        }
    }
}

pub fn generate(node: Node) {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    generate_node(&node);
    println!("  pop rax");
    println!("  ret");
}
