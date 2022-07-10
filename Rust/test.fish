#!/usr/bin/fish

function assert
   set expected $argv[1]
   set input $argv[2]

   cargo run $input > out/temp.s
   cc -o out/temp out/temp.s
   ./out/temp
   set actual $status

   if test "$actual" = "$expected"
      echo "$input => $actual"
   else
      echo "$input => $expected expected, but got $actual"
      exit 1
   end
end

assert 0 0
assert 42 42
echo "OK"