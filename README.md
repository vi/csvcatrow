# csvcatrow 

Tool for concatenating many small csv files into one big table row-wise, analogue of `xsv cat rows`, but handling reordered or different columns.


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

Built it with `cargo install --path .` or download pre-built execitables from [Github Releases]().
