# Redis

## RESP

[RESP](https://redis.io/docs/reference/protocol-spec/): redis serialization protocol

the advantage of redis is that:

- Simple to implement
- Fast to parse
- Human readable

RESP use prefix to tag data type and length

### Base Context of RESP

all data types are that:

- Simple Strings
- Errors
- Integers
- Bulk Strings
- Arrays

RESP is request-response protocol. So cient sender command as Array of Bulk Strings to Redis resevers. And Server response by received.

### Response Prefixed

- Simple strings: the first byte of the reply is "+"
- Errors : the first byte of the reply is "-"
- Integers : the first byte of the reply is ":"
- Bulk Strings : the first byte of the reply is "$"
- Arrays : the first byte of the reply is "\*"
- different parts of the protocol are always terminated with "\r\n"

### Examples of all Data Types

```markdown
# Simple Strings
    "+OK\r\n"

# Errors
    "-ERR unknown command 'helloworld'\r\n"

# Integers
    ":1000\r\n"

# Bulk Strings
    "$5\r\nhello\r\n" => "hello"
    "$0\r\n\r\n" => ""
    "$-1\r\n" => Null(Null Bulk String)

# Arrays
    "*0\r\n" => []
    "*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n" => ["hello","world"]
    "*3\r\n:1\r\n:2\r\n:3\r\n" => [1,2,3]
    "*5\r\n:1\r\n:2\r\n:3\r\n:4\r\n$5\r\nhello\r\n" => [1,2,3,4,"hello"]
    "\*-1\r\n" => Null(Null Array)

# Nested arrays
    *2\r\n
    *3\r\n
    :1\r\n
    :2\r\n
    :3\r\n
    \*2\r\n
    +Hello\r\n
    -World\r\n => [[1,2,3],["Hello", Err("World")]]

# Null elements in Arrays
    _3\r\n
    $5\r\n
    hello\r\n
    $-1\r\n
    $5\r\n
    world\r\n => ["hello",nil,"world"]
    _/
```
