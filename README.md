# RTasks

RTasks is a simple Rust CLI Tool to manage tasks.

## Installation

WIP


## Usage

```bash
./RTasks file password
              ^^^^^^^^optional
```
RTasks saves to a csv file, which holds all the tasks. When an empty or non-existent csv is supplied, a new one of the name will be created.
The password is optional, if supplied at the creation of the file the contents will be Aes256 encrypted.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.