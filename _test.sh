#! /bin/bash
unalias -a

test -d ./sample || mkdir ./sample
<./tests/members.csv cargo run -- id,10-1 id,10-2 >./sample/expect-2ids.csv
diff ./tests/expect-2ids.csv ./sample/expect-2ids.csv
