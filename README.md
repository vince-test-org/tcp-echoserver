## tcp-echoserver

For personal test.

Return based on input

- number: return number - 1
- etc: return itself

### env
- `PORT`
  (required)
### ex)

```sh
$ PORT=65432 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/tcp-echoserver`
Server listening on 0.0.0.0:65432
```
Run server

```sh
$ telnet 0.0.0.0 65432
Trying 0.0.0.0...
Connected to 0.0.0.0.
Escape character is '^]'.
a
a
1
2^CConnection closed by foreign host.
```
