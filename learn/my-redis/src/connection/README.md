# Frame

use "\r\n" separator the serialization orders and warp this command to Frame.

## Function

- array
  - return an empty Frame::Array and init a empty vector;
- push_bulk
  - if type is Frame::Array and push bytes to array or error
- push_int
  - if type is Frame::Array and push u64 to array or error
- check
  - not return Frame, just check and report errors
  - when a order is received parts and we'll parse many times. so use check is more efficient.
- parse
  - return Frame and report errors.
  - parse order when it's received fully.

# Parse

decode Frame to different type orders.

## Example

convert `set foo 123` to a Frame cursor;

# Connect

manager the connections and sender and receive all orders.
