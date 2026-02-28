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

There are two set of programs in the workspace: one using [Solana SDK](https://github.com/anza-xyz/solana-sdk) ("sdk") and another using [pinocchio](https://github.com/anza-xyz/pinocchio) ("pinocchio"). They provide the scaffolding to test and benchmark processor implementations.

```rust
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    core::hint::black_box(Ok(()))
}
```

The workspace uses [mollusk](https://github.com/anza-xyz/mollusk) to run tests and measure compute units. There are 3 benches setup:
* `entrypoint`: executes the program with a variable number of accounts.
* `cpi`: executes the program assuming that it will CPI into the syste, program to create an account.
* `log`: executes the program assuming it will log the lamports of an account.

### Building and Running

A [`Makefile`](https://github.com/febo/playground/blob/main/Makefile) is provided with basic commands to:
* `all`: build all programs.
* `bench <type> [bench name]`: run a specific bench test.
* `build-<type>-<program-name>`: build a program.
* `clean`: remove all build files.
* `clippy`: run `cargo clippy` on the workspace.
* `format`: run `cargo fmt` on the workspace.

To execute a program, it is first necessary to build them:
```bash
make build-pinocchio-accounts
```

To run a `bench` in a particular program:
```bash
make bench pinocchio entrypoint
```

After the execution, mollusk with report the compute units in a `compute_units.md` located at `./target/benches`. For the `entrypoint` bench, the file contents would look like:
```
| Name         | CUs | Delta   |
|--------------|-----|---------|
| Account (0)  | 13  | - new - |
| Account (1)  | 17  | - new - |
| Account (2)  | 17  | - new - |
| Account (3)  | 37  | - new - |
| Account (4)  | 45  | - new - |
| Account (8)  | 78  | - new - |
| Account (16) | 143 | - new - |
| Account (32) | 261 | - new - |
| Account (64) | 504 | - new - |
```

For `cpi`:
```
| Name                   | CUs  | Delta   |
|------------------------|------|---------|
| system_program::create | 1281 | - new - |
```

For `log`:
```
| Name | CUs  | Delta   |
|------|------|---------|
| log  | 447  | - new - |
```

When you make a modification or run a different type of program ("pinocchio" or "sdk") but execute the same bench test, the "Delta" column will show the difference in CUs compared to the previous run.

## Adding a bench test

To add a new bench test, go to `tests/benches/setup` folder and edit the `mod.rs` file with a new `MolluskComputeUnitBencher`. Then call the new bench from the test runner of each program – these are located in `tests/benches`.

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
