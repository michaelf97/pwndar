# PWNDAR

Pwndar is a Rust Command Line Tool that interacts with the https://haveibeenpwned.com/ API.

## Installation

Use the package manager [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install pwndar.

```bash
cargo install pwndar
```

## API KEY

An API Key is needed to interact with https://haveibeenpwned.com/ API.

Place a file called keys.yaml in the working directory. Keys.yaml should take the following format;
```yaml
hibp: [key]
```
This is temporary to allow integration of similiar services into the pwndar tool.

### Future

Future versions will implement a key command line argument and/or config file.

## Usage

```bash
#> pwndar test@email.com
[BREACH RESULTS]:  COMPROMISED!
        0: BAD_WEBSITE_1
        1: BAD_WEBSITE_2

[PASTE RESULTS]:  CLEAR!

#> pwndar test@email.com
 _______           _        ______   _______  _______ 
(  ____ )|\     /|( (    /|(  __  \ (  ___  )(  ____ )
| (    )|| )   ( ||  \  ( || (  \  )| (   ) || (    )|
| (____)|| | _ | ||   \ | || |   ) || (___) || (____)|
|  _____)| |( )| || (\ \) || |   | ||  ___  ||     __)
| (      | || || || | \   || |   ) || (   ) || (\ (
| )      | () () || )  \  || (__/  )| )   ( || ) \ \__
|/       (_______)|/    )_)(______/ |/     \||/   \__/

Pwndar 0.1
Michael Forret <michael.forret@quorumcyber.com>

USAGE:
    pwndar.exe [FLAGS] <EMAIL>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information (not implemented yet)
    -v, --verbose    Display verbose output (not implemented yet)

ARGS:
    <EMAIL>    Sets the email to parse
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)