use crate::node::Node;
pub use self::parse_c::parse;

peg::parser!( pub grammar parse_c() for str {
    pub rule parse() -> Node = expr();

    rule expr() -> Node
      = _ n:number() _ "+" _ r:expr() { Node::add(Node::Number {value: n}, r) }
      / _ n:number() { Node::Number {value: n} }

    rule number() -> i64
        = n:$(['0'..='9']+)
        { n.parse().unwrap() }

    rule _ = [' ' | '\t' | '\n']*
});
