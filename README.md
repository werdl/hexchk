## hexchk
> a small binary pretty colored hex viewer
## Requirements
- `jq`
- `rustup`
- `upx`

## What is this?
This is your standard hex viewer, with some colors for fun. The real kicker, though, is the binary size. In Rust, I have certainly noticed that by default, the binaries are pretty chunky. So, a lot of optimization has been done to get the binary size down to a minimum. The final binary size is 23572 bytes. This is better than the standard `hexdump` utility, which (on my system), stands at 59720 bytes. Admittedly, `xxd` is slightly smaller at 18648 bytes, but it doesn't have the pretty colors.

The binary size is very small, and since the colors are also present, performance takes a hit. It takes around 10 times longer than its competitors - but this is owing to the colors. I know this as I wrote a version in C (and got the binary size down to 6112 bytes), and it took almost the same time as the Rust one (slightly faster), as they both had colors. The C one is provided at `hexchk.c`, in the root directory. They are also partly slower as I wrote and optimized them in about half an hour, rather than taking as much time and effort as `hexdump` and `xxd` have.

Another reason why they are both so slow is their optimizations for binary size, rather than speed. Various techniques have been applied, but some of the linker ones, and the choice of optimizations have been chosen to reduce the binary size, rather than speed. This is why the binary is so small - but also why it is so slow.

You might also say, "`xxd` is smaller!". This is true, but the C version is a third of the size of `xxd` and uses the same logic. What we are really seeing here is the difference between C and Rust. I took the same two binaries, and thrashed them with the same level of extreme optimization, and the C one came out slightly faster, and also ~4 times smaller. 

No hate to Rust though, I love it as a language. It's just that the compiler is still in its early days - C has had umpteen years to develop, and `gcc` has been around since 1987.

Also, the Rust code is A LOT easier to read than the C code.
## Performance
23572B is `hexchk` itself.

152B is `Cargo.toml`.

3804432B is `/bin/perl`.

| Tool | File Size | Time |
|------|-----------|------|
| `hexchk` | 23572B | 0.488s |
| `hexchk_c` | 23572B | 0.532s |
| `hexdump` | 23572B | 0.044s |
| `xxd` | 23572B | 0.030s |
| `hexchk` | 152B | 0.013s |
| `hexchk_c` | 152B | 0.008s |
| `hexdump` | 152B | 0.005s |
| `xxd` | 152B | 0.005s |
| `hexchk` | 3804432B | 225.956s |
| `hexchk_c` | 3804432B | 208.571s |
| `hexdump` | 3804432B | 35.036s |
| `xxd` | 3804432B | 32.519s |

![image](https://github.com/werdl/hexchk/assets/116349156/3bdbb81b-1079-4230-af24-31af204b64fb)

