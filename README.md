# Wipers
Automatic Test Writing with LLMs

## Setup

working on it.....


## Configuration

Configuration of Wipers can be achieved using the wipers.toml file. To start, set your openai api key and org_id (if applicable) like so:

```
[openai]
key = "<MY_OPEN_AI_KEY>"
org_id = "<MY_ORG_ID>"
```

## Run Wipers

To run Wipers

### Binary

```shell
wipers -f "<MY_FILE>"
```

### Debug

```shell
cargo run -- -f "<MY_FILE>"
```


## Lock Tests

There are several reasons why you might not want to re-write tests for a given file. To stop Wipers from overwriting a given test file please add the file path string to the lockfiles array in your wipers.toml file like so:

```
[lock]
lockfiles = ["./tests/test_my_file.py",""./tests/test_functions.py]
```

## FAQ

waiting on questions.....


## Todos

###  Basic Functionality
- [X] File loading
- [X] File parsing
- [X] OpenAI logic 
- [X] OpenAI response processing
- [X] Appliction Writes tests
- [ ] Test file locking
- [X] Toml Configuration
- [ ] CLI args

### Unittests
- [X] config.rs
- [X] file.rs
- [X] parse.rs
- [ ] postprocess.rs
- [ ] write.rs  

### File support
- [X] Add support for Python files
- [X] Add support for Javascript files
- [ ] Add support for Rust files
- [ ] Add support for Go files
- [ ] Add support for C++ files
- [ ] Add support for 

### LLMS

- [ ] Add support for other LLM providers


### CI

- [X] cargo test

### Docs 

- [ ] Add Description section
- [ ] Finish README Setup Section
- [ ] Finish README FAQ Section
- [ ] Write CONTRIBUTING.md

