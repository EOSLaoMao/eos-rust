## How to build
```
cargo build --all
```

## How to use
``` 
./target/debug/eoc-cli --help

USAGE:
    eos-cli [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Sets the level of verbosity

SUBCOMMANDS:
    account        get account info
    balance        get account balance
    connections    get connections
    createkey      create EOS key pair
    help           Prints this message or the help of the given subcommand(s)
    info           get info from eos blockchain
```
## Example
``` 
    ./target/debug/eoc-cli createkeys

    Terminal Output: 
    private: 5K2RgdF6vKC6CQNpqebxxxxx.........
    public: EOS6v9863knSSJ8M9umSxxxxx..........     

```
The project just started, any contribution will appreciated.


