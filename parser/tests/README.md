# ***Testing***

While *extremely* important, testing is not really a major concern for this project as it's a learning exercise. 

Nonetheless, the tests included are for parsing all the examples in the [resources folder] in the Hail codebase (with the exception of the [backward compatibility] sections 1.1.0 and 1.0.0).

> An important note, however, is that the parser makes an effort to report an error any time something unexpected happens with the hope to have few silent errors.

Manual inspection of the data compared to the Python Hail extension has been done throughout the development cycle.


[resources folder]: https://github.com/hail-is/hail/tree/main/hail/src/test/resources
[backward compatibility]: https://github.com/hail-is/hail/tree/main/hail/src/test/resources/backward_compatability