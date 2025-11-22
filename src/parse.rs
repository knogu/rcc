use crate::node::Node;
pub use self::parse_c::parse;

peg::parser!( pub grammar parse_c() for str {
    pub rule parse() -> Node = expr();

    rule expr() -> Node
      = arithmetic()

    rule arithmetic() -> Node = precedence!{
        x:(@) _ "+" _ y:@ { Node::add(x, y) }
        x:(@) _ "-" _ y:@ { Node::sub(x, y) }
        --
        n:number() { n }
        "(" e:arithmetic() ")" { e }
    }

    rule number() -> Node
        = n:$(['0'..='9']+)
        { Node::Number{value: n.parse().unwrap()} }

    rule _ = [' ' | '\t' | '\n']*
});
