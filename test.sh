cargo run -- 42 > tmp.s
cc -o tmp tmp.s
./tmp
echo $?
