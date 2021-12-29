# xxd-rs [![crates-io](https://img.shields.io/crates/v/xxd-rs.svg)](https://crates.io/crates/xxd-rs)
A rust based reimplementation of [Juergen Weigert's](jnweiger@informatik.uni-erlangen.de) hexdump utility, xxd.

## Mission statement
This project was created to learn rust, therefore there is no perf
If you wanna use the project or contribute feel free, but please take note
of the goal(s) and non goals so you won't waste your time or get frustrated.

### Goals
1. Learn rust
2. Provide a rust based replacement for xxd
3. Strive towards a clean rust code base
    - rustfmt
    - Add/refactor towards common rust idioms
4. Usability
    - Provide user friendly defaults
    - Provide clear and well documented command line tools
5. Continuously improve the code base


### Non Goals
What this Project isn't about

1. Implement the fastest dump utility out there
    - If you are looking for a performant implementation of xxd
      checkout [go-xxd](https://github.com/felixge/go-xxd)

## Usage
```
USAGE:
    xxd-rs [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --length <length>          Amount of bytes which shall be read
    -o, --output-file <outfile>    File to which the output will be written (default: stdout)
    -s, --seek <seek>              Offset in the file where to start reading

SUBCOMMANDS:
    dump        Dumps an input file in the appropriate output format
    generate    Generates a source file containing the specified file as array
    help        Prints this message or the help of the given subcommand(s)
```

## Examples
1. Dump file
```
user@host:~$ xxd-rs dump file.txt
```

2. Dump file with 16 bit word size
```
user@host:~$ xxd-rs generate -g 2 file.txt
```

3. Dump 1024 bytes of file file starting at offset 128
```
user@host:~$ xxd-rs dump -s 128 -l 1024 file.txt
```

4. Generate cpp header file containing file in an cpp array
```
user@host:~$ xxd-rs generate -t cpp file.txt
```

## Migration/Compatibility
Be aware that the output formats (especially the default) of hexdump, xxd, od, and xxd-rs differ.

1. Dump file
    1. xxd
    ```
    user@host:~$ xxd file
    ```
    2. od
    ```
    user@host:~$ od file
    ```
    3. hexdump
    ```
    user@host:~$ xxd-rs dump file
    ```
    4. xxd-rs
    ```
    user@host:~$ xxd-rs dump file
    ```

## Screenshots
### hex dump
```
user@host:~$ xxd-rs dump Cargo.toml
00000000: 5b70 6163 6b61 6765 5d0a 6e61 6d65 203d  [package].name =
00000010: 2022 7878 642d 7273 220a 6465 7363 7269   "xxd-rs".descri
00000020: 7074 696f 6e20 3d20 2241 2072 7573 7420  ption = "A rust
00000030: 636c 6f6e 6520 6f66 2078 7864 220a 7265  clone of xxd".re
00000040: 706f 7369 746f 7279 203d 2022 6874 7470  pository = "http
00000050: 733a 2f2f 6769 7468 7562 2e63 6f6d 2f4e  s://github.com/N
00000060: 6963 6f72 6574 7469 2f78 7864 2d72 7322  icoretti/xxd-rs"
00000070: 0a72 6561 646d 6520 3d20 2252 4541 444d  .readme = "READM
00000080: 452e 6d64 220a 6175 7468 6f72 7320 3d20  E.md".authors =
00000090: 5b22 4e69 636f 6c61 2043 6f72 6574 7469  ["Nicola Coretti
000000A0: 203c 6e69 636f 2e63 6f72 6574 7469 4067   <nico.coretti@g
000000B0: 6d61 696c 2e63 6f6d 3e22 5d0a 6b65 7977  mail.com>"].keyw
000000C0: 6f72 6473 203d 205b 2278 7864 222c 2022  ords = ["xxd", "
000000D0: 6865 782d 6475 6d70 225d 0a6c 6963 656e  hex-dump"].licen
000000E0: 7365 2d66 696c 6520 3d20 224c 4943 454e  se-file = "LICEN
000000F0: 5345 220a 7665 7273 696f 6e20 3d20 2230  SE".version = "0
00000100: 2e32 2e31 220a 6275 696c 6420 3d20 2262  .2.1".build = "b
00000110: 7569 6c64 2e72 7322 0a0a 5b62 6164 6765  uild.rs"..[badge
00000120: 735d 0a61 7070 7665 796f 7220 3d20 7b20  s].appveyor = {
00000130: 7265 706f 7369 746f 7279 203d 2022 4e69  repository = "Ni
00000140: 636f 7265 7474 692f 7878 642d 7273 222c  coretti/xxd-rs",
00000150: 2062 7261 6e63 6820 3d20 226d 6173 7465   branch = "maste
00000160: 7222 2c20 7365 7276 6963 6520 3d20 2267  r", service = "g
00000170: 6974 6875 6222 207d 0a74 7261 7669 732d  ithub" }.travis-
00000180: 6369 203d 207b 2072 6570 6f73 6974 6f72  ci = { repositor
00000190: 7920 3d20 224e 6963 6f72 6574 7469 2f78  y = "Nicoretti/x
000001A0: 7864 2d72 7322 2c20 6272 616e 6368 203d  xd-rs", branch =
000001B0: 2022 6d61 7374 6572 2220 7d0a 0a5b 6275   "master" }..[bu
000001C0: 696c 642d 6465 7065 6e64 656e 6369 6573  ild-dependencies
000001D0: 5d0a 636c 6170 203d 2022 322e 3136 2e32  ].clap = "2.16.2
000001E0: 220a 0a5b 6c69 625d 0a6e 616d 6520 3d20  "..[lib].name =
000001F0: 2278 7864 220a 7061 7468 203d 2022 7372  "xxd".path = "sr
00000200: 632f 7878 642f 6c69 622e 7273 220a 0a5b  c/xxd/lib.rs"..[
00000210: 5b62 696e 5d5d 0a6e 616d 6520 3d20 2278  [bin]].name = "x
00000220: 7864 2d72 7322 0a74 6573 7420 3d20 6661  xd-rs".test = fa
00000230: 6c73 650a 646f 6320 3d20 6661 6c73 650a  lse.doc = false.
00000240: 0a5b 6465 7065 6e64 656e 6369 6573 5d0a  .[dependencies].
00000250: 6e6f 6d20 3d20 2233 2e30 2e30 220a 6572  nom = "3.0.0".er
00000260: 726f 722d 6368 6169 6e20 3d20 2230 2e39  ror-chain = "0.9
00000270: 2e30 220a 7465 726d 2d70 6169 6e74 6572  .0".term-painter
00000280: 203d 2022 302e 322e 3322 0a63 6c61 7020   = "0.2.3".clap
00000290: 3d20 2232 2e31 362e 3222 0a
```

```
user@host:~$ xxd-rs dump -f Hex Cargo.toml
00000000: 5B70 6163 6B61 6765 5D0A 6E61 6D65 203D  [package].name =
00000010: 2022 7878 642D 7273 220A 6465 7363 7269   "xxd-rs".descri
00000020: 7074 696F 6E20 3D20 2241 2072 7573 7420  ption = "A rust
00000030: 636C 6F6E 6520 6F66 2078 7864 220A 7265  clone of xxd".re
00000040: 706F 7369 746F 7279 203D 2022 6874 7470  pository = "http
00000050: 733A 2F2F 6769 7468 7562 2E63 6F6D 2F4E  s://github.com/N
00000060: 6963 6F72 6574 7469 2F78 7864 2D72 7322  icoretti/xxd-rs"
00000070: 0A72 6561 646D 6520 3D20 2252 4541 444D  .readme = "READM
00000080: 452E 6D64 220A 6175 7468 6F72 7320 3D20  E.md".authors =
00000090: 5B22 4E69 636F 6C61 2043 6F72 6574 7469  ["Nicola Coretti
000000A0: 203C 6E69 636F 2E63 6F72 6574 7469 4067   <nico.coretti@g
000000B0: 6D61 696C 2E63 6F6D 3E22 5D0A 6B65 7977  mail.com>"].keyw
000000C0: 6F72 6473 203D 205B 2278 7864 222C 2022  ords = ["xxd", "
000000D0: 6865 782D 6475 6D70 225D 0A6C 6963 656E  hex-dump"].licen
000000E0: 7365 2D66 696C 6520 3D20 224C 4943 454E  se-file = "LICEN
...
```

### octal dump
```
user@host:~$ xxd-rs dump -f oct Cargo.toml
00000000: 133160 141143 153141 147145 135012 156141 155145 040075  [package].name =
00000010: 040042 170170 144055 162163 042012 144145 163143 162151   "xxd-rs".descri
00000020: 160164 151157 156040 075040 042101 040162 165163 164040  ption = "A rust
00000030: 143154 157156 145040 157146 040170 170144 042012 162145  clone of xxd".re
00000040: 160157 163151 164157 162171 040075 040042 150164 164160  pository = "http
00000050: 163072 057057 147151 164150 165142 056143 157155 057116  s://github.com/N
00000060: 151143 157162 145164 164151 057170 170144 055162 163042  icoretti/xxd-rs"
00000070: 012162 145141 144155 145040 075040 042122 105101 104115  .readme = "READM
00000080: 105056 155144 042012 141165 164150 157162 163040 075040  E.md".authors =
00000090: 133042 116151 143157 154141 040103 157162 145164 164151  ["Nicola Coretti
000000A0: 040074 156151 143157 056143 157162 145164 164151 100147   <nico.coretti@g
000000B0: 155141 151154 056143 157155 076042 135012 153145 171167  mail.com>"].keyw
000000C0: 157162 144163 040075 040133 042170 170144 042054 040042  ords = ["xxd", "
000000D0: 150145 170055 144165 155160 042135 012154 151143 145156  hex-dump"].licen
000000E0: 163145 055146 151154 145040 075040 042114 111103 105116  se-file = "LICEN
...
```

### binary dump
```
user@host:~$ xxd-rs dump -f bin -g 2 -c 4 Cargo.toml
00000000: 0101101101110000 0110000101100011 0110101101100001 0110011101100101  [package
00000008: 0101110100001010 0110111001100001 0110110101100101 0010000000111101  ].name =
00000010: 0010000000100010 0111100001111000 0110010000101101 0111001001110011   "xxd-rs
00000018: 0010001000001010 0110010001100101 0111001101100011 0111001001101001  ".descri
00000020: 0111000001110100 0110100101101111 0110111000100000 0011110100100000  ption =
00000028: 0010001001000001 0010000001110010 0111010101110011 0111010000100000  "A rust
00000030: 0110001101101100 0110111101101110 0110010100100000 0110111101100110  clone of
00000038: 0010000001111000 0111100001100100 0010001000001010 0111001001100101   xxd".re
00000040: 0111000001101111 0111001101101001 0111010001101111 0111001001111001  pository
00000048: 0010000000111101 0010000000100010 0110100001110100 0111010001110000   = "http
00000050: 0111001100111010 0010111100101111 0110011101101001 0111010001101000  s://gith
00000058: 0111010101100010 0010111001100011 0110111101101101 0010111101001110  ub.com/N
00000060: 0110100101100011 0110111101110010 0110010101110100 0111010001101001  icoretti
00000068: 0010111101111000 0111100001100100 0010110101110010 0111001100100010  /xxd-rs"
...
```

### decimal dump
```
user@host:~$ xxd-rs dump -f dec Cargo.toml
00000000: 091112 097099 107097 103101 093010 110097 109101 032061  [package].name =
00000010: 032034 120120 100045 114115 034010 100101 115099 114105   "xxd-rs".descri
00000020: 112116 105111 110032 061032 034065 032114 117115 116032  ption = "A rust
00000030: 099108 111110 101032 111102 032120 120100 034010 114101  clone of xxd".re
00000040: 112111 115105 116111 114121 032061 032034 104116 116112  pository = "http
00000050: 115058 047047 103105 116104 117098 046099 111109 047078  s://github.com/N
00000060: 105099 111114 101116 116105 047120 120100 045114 115034  icoretti/xxd-rs"
00000070: 010114 101097 100109 101032 061032 034082 069065 068077  .readme = "READM
00000080: 069046 109100 034010 097117 116104 111114 115032 061032  E.md".authors =
00000090: 091034 078105 099111 108097 032067 111114 101116 116105  ["Nicola Coretti
000000A0: 032060 110105 099111 046099 111114 101116 116105 064103   <nico.coretti@g
000000B0: 109097 105108 046099 111109 062034 093010 107101 121119  mail.com>"].keyw
000000C0: 111114 100115 032061 032091 034120 120100 034044 032034  ords = ["xxd", "
000000D0: 104101 120045 100117 109112 034093 010108 105099 101110  hex-dump"].licen
000000E0: 115101 045102 105108 101032 061032 034076 073067 069078  se-file = "LICEN
...
```

### plain dump
```
user@host:~$ xxd-rs dump -g 4 -p Cargo.toml
5b7061636b6167655d0a6e616d65203d20227878642d7273220a646573637269
7074696f6e203d202241207275737420636c6f6e65206f6620787864220a7265
706f7369746f7279203d202268747470733a2f2f6769746875622e636f6d2f4e
69636f72657474692f7878642d7273220a726561646d65203d2022524541444d
452e6d64220a617574686f7273203d205b224e69636f6c6120436f7265747469
203c6e69636f2e636f726574746940676d61696c2e636f6d3e225d0a6b657977
6f726473203d205b22787864222c20226865782d64756d70225d0a6c6963656e
73652d66696c65203d20224c4943454e5345220a76657273696f6e203d202230
2e322e31220a6275696c64203d20226275696c642e7273220a0a5b6261646765
735d0a6170707665796f72203d207b207265706f7369746f7279203d20224e69
636f72657474692f7878642d7273222c206272616e6368203d20226d61737465
72222c2073657276696365203d202267697468756222207d0a7472617669732d
6369203d207b207265706f7369746f7279203d20224e69636f72657474692f78
78642d7273222c206272616e6368203d20226d617374657222207d0a0a5b6275
696c642d646570656e64656e636965735d0a636c6170203d2022322e31362e32
220a0a5b6c69625d0a6e616d65203d2022787864220a70617468203d20227372
632f7878642f6c69622e7273220a0a5b5b62696e5d5d0a6e616d65203d202278
78642d7273220a74657374203d2066616c73650a646f63203d2066616c73650a
0a5b646570656e64656e636965735d0a6e6f6d203d2022332e302e30220a6572
726f722d636861696e203d2022302e392e30220a7465726d2d7061696e746572
203d2022302e322e33220a636c6170203d2022322e31362e32220a
```
