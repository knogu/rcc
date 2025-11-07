pub use self::parse_c::parse;

peg::parser!( pub grammar parse_c() for str {
    pub rule parse() -> i64 = number();

    rule number() -> i64
        = n:$(['0'..='9']+)
        { n.parse().unwrap() }
});
