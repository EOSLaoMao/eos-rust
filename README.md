## How to build
```
cargo build --all
```

## How to use
``` 
./target/debug/eoc-cli 


USAGE:
    eos-cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    get       Retrieve various items and information from the blockchain
    help      Prints this message or the help of the given subcommand(s)
    wallet    Interact with local wallet
```

## Example
``` 
    ./target/debug/eoc-cli wallet create_key

    Terminal Output: 
    private: 5K2RgdF6vKC6CQNpqebxxxxx.........
    public: EOS6v9863knSSJ8M9umSxxxxx..........     

```
The project just started, any contribution will be appreciated.


