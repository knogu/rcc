use crate::node::Node;
pub use self::parse_c::parse;

peg::parser!( pub grammar parse_c() for str {
    pub rule parse() -> Node = add();

    #[cache_left_rec]
    rule add() -> Node
      = _ l:add() _ "+" _ r:add() { Node::add(l, r) }
      / _ n:number() { Node::Number {value: n} }

    rule number() -> i64
        = n:$(['0'..='9']+)
        { n.parse().unwrap() }

    rule _ = [' ' | '\t' | '\n']*
});
