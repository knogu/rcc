#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  cargo run -- "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 2 '5-3'
assert 2 '10 - 5 - 3'
assert 8 '3+5'
assert 25 '10+15'
assert 27 '10+15 + 2'
assert 42 42
assert 42 42

echo OK
