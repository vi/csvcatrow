# csvcatrow 

Tool for concatenating many small csv files into one big table row-wise, analogue of [`xsv cat rows`][1], but handling reordered or different columns.


```
$ cat 1.csv
a,b
1,2

$ cat 2.csv
b,a
2,1

$ cat 3.csv
a,b,c
1,2,3

$ csvcatrow 1.csv 2.csv 3.csv
a,b,c
1,2,
1,2,
1,2,3
```

Install it with `cargo install --path .` (or `cargo install csvcatrow`) or download pre-built execitables from [Github Releases][2].

[1]:https://github.com/BurntSushi/xsv
[2]:https://github.com/vi/csvcatrow/releases/

# See also

[qsv's `cat rowskey`][3] subcommand.

[3]:https://github.com/jqnatividad/qsv/blob/fd4dd17272c06e085045929d59620fc0a5c4b04e/src/cmd/cat.rs#L18
