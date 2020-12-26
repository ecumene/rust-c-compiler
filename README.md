# Learning to write compilers

[Following this tutorial](https://norasandler.com/2017/11/29/Write-a-Compiler.html)

## Running

### Compile

```
cargo run -- test.c > test.s
gcc test.s -o test
```

### Run and Print Exit Code

```
./test
echo $?
```

## Testing Code

```
cargo test
```

or

```
cargo watch -x test
```

to hotreload
