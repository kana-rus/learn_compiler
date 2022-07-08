#!/usr/bin/fish

function assert
   set expected $argv[1]
   set input $argv[2]

#   echo "executed"
#   echo $input
#   if test "$expected" = "$input"
#      echo "equal"
#   end
   ./9cc $input > temp.s
   cc -o temp temp.s
   ./temp
   set actual $status

   if test "$actual" = "$expected"
      echo "$input => $actual"
   else
      echo "$input => $expected expected, but got $actual"
      exit 1
   end
end

assert 21 "5+20-4"
echo "OK"