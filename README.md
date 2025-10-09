<h1 align="center">
  <code>playground</code>
</h1>
<p align="center">
  <img width="400" alt="playground" src="https://github.com/user-attachments/assets/9ae2b22c-8ebb-49d5-bb18-039719de7618" />
</p>

<p align="center">
  A space to play with programs.
</p>

## Getting started

There are two program in the workspace: one using [Solana SDK](https://github.com/anza-xyz/solana-sdk) ("sdk") and another using [pinocchio](https://github.com/anza-xyz/pinocchio) ("pinocchio"). They provide the scaffolding to test and benchmark processor implementations.

```rust
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    core::hint::black_box(Ok(()))
}
```

The workspace uses [mollusk](https://github.com/anza-xyz/mollusk) to run tests and measure compute units. There are 3 benches setup:
* `run_accounts`: executes the program with a variable number of accounts.
* `run_cpi`: executes the program assuming that it will CPI into the syste, program to create an account.
* `run_log`: executes the program assuming it will log the lamports of an account.

Since it does not use instruction discriminators, it is necessary to change the processor implementation to run different benches.

### Building and Running

A [`Makefile`](https://github.com/febo/playground/blob/main/Makefile) is provided with basic commands to:
* `bench`: run a specific bench test against a program.
* `build`: build both programs.
* `clean`: remove all build files.
* `clippy`: run `cargo clippy` on the workspace.
* `format`: run `cargo fmt` on the workspace.

To execute a program, it is first necessary to build them:
```bash
make build
```

To run a `bench` in a particular program:
```bash
make bench <program> <bench name>
```

For example, to run the `run_accounts` bench on the "pinocchio" program:
```bash
make bench pinocchio run_account
```

After the execution, mollusk with report the compute units in a `compute_units.md` located at `./target/benches`. For the `run_accounts` bench, the file contents would look like:
```
| Name         | CUs | Delta   |
|--------------|-----|---------|
| Account (1)  | 17  | - new - |
| Account (2)  | 17  | - new - |
| Account (3)  | 37  | - new - |
| Account (4)  | 45  | - new - |
| Account (8)  | 78  | - new - |
| Account (16) | 143 | - new - |
| Account (32) | 261 | - new - |
| Account (64) | 504 | - new - |
```

For `run_cpi`:
```
| Name                   | CUs  | Delta   |
|------------------------|------|---------|
| system_program::create | 1281 | - new - |
```

For `run_log`:
```
| Name | CUs  | Delta   |
|------|------|---------|
| log  | 447  | - new - |
```

When you make modification or run a different program but execute the same bench test, the "Delta" column will show the difference in CUs compared to the previous run.

## Adding a bench test

To add a new bench test, go to `tests/benches/setup` folder and edit the `mod.rs` file with a new `MolluskComputeUnitBencher`. Then call the new bench from the test runner of each program – these are located in `tests/benches`.

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
