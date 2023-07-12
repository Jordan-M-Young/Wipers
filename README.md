# Wipers
Automatic Test Writing with LLMs

## Description

Writing tests for your code can be annoying, use wipers to leverage openai models to write your tests for you! 

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
wipers -f "<MY_FILE>" -o "<MY_TEST_DIRECTORY>"
```

### Debug

```shell
cargo run -- -f "<MY_FILE>" -o "<MY_TEST_DIRECTORY>"
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

###  Basic Functionality [X]
- [X] File loading
- [X] File parsing
- [X] OpenAI logic 
- [X] OpenAI response processing
- [X] Appliction Writes tests
- [X] Basic Test file locking
- [X] Toml Configuration
- [X] Basic CLI args

### Unittests
- [X] config.rs
- [X] file.rs
- [X] parse.rs
- [ ] postprocess.rs
- [ ] write.rs  
- [X] cli.rs
- [ ] lock.rs 

### File support
- [X] Add support for Python files
- [X] Add support for Javascript files
- [ ] Add support for Rust files
- [ ] Add support for Go files
- [ ] Add support for C++ files
- [ ] Add support for.....

### LLMS

- [ ] Add support for other LLM providers


### CI
- [X] Commit Hooks
- [X] Test Pipeline

### Docs 

- [ ] Add Description section
- [ ] Finish README Setup Section
- [ ] Finish README FAQ Section
- [ ] Write CONTRIBUTING.md



## Reliability

If you've ever used wipers to clean the windshield of a dirty car you might have noticed they dont get the job 100% done. Sometimes you'll need to stop and clean the windshield by hand. Likewise, Wipers is great for churning out tests for your code, but make sure to go over the tests that you write with this tool. 