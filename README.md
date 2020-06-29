# xjr

xjr is a tool to convert xSV to JSON Lines.

## Usage

```
Usage: xjr [options] [FILE]

Options:
    -s, --sep SEP       line separator. not split if empty
    -k, --keys KEYS     json object keys
        --header        consider the first line as a header
    -v, --version       print version and exit
    -h, --help          print usage and exit
```

```
$ printf 'foo,bar\nbar,zoo' | xjr
["foo","bar"]
["bar","zoo"]

$ printf 'foo\tbar\nbar\tzoo' | xjr -s '\t'
["foo","bar"]
["bar","zoo"]

$ printf 'foo,bar\nbar,zoo' | xjr -s ''
["foo,bar"]
["bar,zoo"]

$ printf 'foo,bar\nbar,zoo' > data.csv
$ xjr data.csv
["foo","bar"]
["bar","zoo"]

$ printf 'foo,bar\nbar,zoo' | xjr -k a,b
{"a":"foo","b":"bar"}
{"a":"bar","b":"zoo"}

$ printf "foo,bar\nzoo,baz\n1,2" | xjr --header
{"foo":"zoo","bar":"baz"}
{"foo":"1","bar":"2"}
```
