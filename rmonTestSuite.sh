 #!/bin/bash
 /usr/bin/nodemon -q -e rs -x "rustc --test $* -o .binary && ./.binary"
