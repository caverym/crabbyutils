# basename

basename in Rust, version 0.1

```
Usage: basename NAME [SUFFIX]
or: basename OPTION... NAME...
Print NAME with leading directory removed
If specified, also remove SUFFIX.

        
    -a, --multiple      support multiple directory as NAME
    -s, --suffix        remove following string as SUFFIX; implies -a
    --help              display this help message
    --version           display version information
```

I did not implement `-z` because I see no real use for it. If you do, create a pull request.
