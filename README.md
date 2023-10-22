# rust-new-project-template
[![Build](https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/actions/workflows/build.yml/badge.svg)](https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/actions/workflows/build.yml)

[![Linting](https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/actions/workflows/lint.yml)

[![Tests](https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/actions/workflows/tests.yml)

[![Rustfmt](https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/actions/workflows/rustfmt.yml)

[![Build binary release](https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/actions/workflows/release.yml)


## Overview

This project demonstrates CRUD operations on the SQLite database. I use Github Copilot to get those CRUD functions and correct some small mistakes.

### Create Table
<img width="548" alt="image" src="https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/assets/108935314/eadc1e15-5091-41cb-ad8f-41da2d750edf">

### Delete Table
<img width="560" alt="image" src="https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/assets/108935314/81b645d5-808a-40e0-bcf5-3be813a75c82">

### Insert Data
<img width="599" alt="image" src="https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/assets/108935314/95f8a0ad-0e91-4919-8204-6fe8f0d6ba94">

### Read Table
<img width="793" alt="image" src="https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/assets/108935314/35386c45-268d-432f-a0b5-ae62864ee7ab">

### Updata Data
<img width="812" alt="image" src="https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/assets/108935314/8323b85c-c4ce-47f6-8b72-6945ef30232a">

### Delete Data
<img width="664" alt="image" src="https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/assets/108935314/e2e03b4d-a1f0-4f22-9a20-ba5330233b0e">


## Dependencies

`rusqlite = "0.26"`
You can find dependencies in Cargo.toml file

## Run
* Check Rust version: `make rust-version`
* Compile the current package: `make build`
* Format code: `make format`
* Lint code: `make lint`
* Test code: `make test`
* Run the current package: `make run`
* puts the resulting binary in target/release instead of target/debug: `make release`
  


## Result
The process is as following: 
1. Create a table 
2. Insert 2 data points then read the current table
3. Update data point 1 and read the current table
4. Delete data point 1 and then read the current table
<img width="773" alt="image" src="https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/assets/108935314/2a879143-5bb4-4280-ac8e-4e1826da74a8">

Then `make run`: 
<img width="1011" alt="image" src="https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/assets/108935314/6a9226ea-38f2-4b5a-86c1-c32fc7a147a0">

Correct Result!

We can also try `make all` which is to compile the current package, and then format, lint, test and run our code.

### Test Result:
<img width="1052" alt="image" src="https://github.com/nogibjj/IDS706_IndividualProject2_Mutian/assets/108935314/01d30680-f04d-434e-9465-e29a711ef26f">


## Copilot

Copilot helped me to get those CRUD functions defined in lib.rs.

