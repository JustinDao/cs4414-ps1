Title: Problem Set 1 Answers
==========
Author: Justin Dao
==========

1.
-----

User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/32.0.1700.77 Safari/537.36

2.
-----

Concurrancy issues could be a reason why Rust thinks modifying a global variable in this way is unsafge. If two people access the server at the same time, the value of the global variable could not be what is expected, or if two people write this value at the same time, it could give an error. The location in memory could be reading and writing at the same time, which could cause some major problems. 