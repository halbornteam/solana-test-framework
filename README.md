# solana-test-framework

`solana-test-framework` extends [`BanksClient`](https://docs.rs/solana-banks-client/latest/solana_banks_client/struct.BanksClient.html), [`RpcClient`](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html), [`ProgramTest`](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTest.html) and [`ProgramTestContext`](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTestContext.html) with several convenience methods. It supports both external clusters and simulated runtime.

&nbsp;
&nbsp;

## Setup
This framework supports:

- Solana v1.9 and Anchor v0.24.2
- Solana v1.10 and Anchor v0.25.0
- Solana v1.11
- Solana v1.12
- Solana v1.13
- Solana v1.14 and Anchor v0.26.0
- Solana v1.16 and Anchor v0.28.0
- Solana v1.18 and Anchor v0.30.0


To use it in your project,

1. add one of the following in your `Cargo.toml`:

    - Solana ~1.9: `solana-test-framework = { git = "https://github.com/halbornlabs/solana-test-framework", branch = "solana1.9"}`
    - Solana ~1.10: `solana-test-framework = { git = "https://github.com/halbornlabs/solana-test-framework", branch = "solana1.10" }`
    - Solana ~1.11: `solana-test-framework = { git = "https://github.com/halbornlabs/solana-test-framework", branch = "solana1.11" }`
    - Solana ~1.12: `solana-test-framework = { git = "https://github.com/halbornlabs/solana-test-framework", branch = "solana1.12" }`
    - Solana ~1.13: `solana-test-framework = { git = "https://github.com/halbornlabs/solana-test-framework", branch = "solana1.13" }`
    - Solana ~1.14: `solana-test-framework = { git = "https://github.com/halbornlabs/solana-test-framework", branch = "solana1.14" }`
     - Solana ~1.16: `solana-test-framework = { git = "https://github.com/halbornlabs/solana-test-framework", branch = "solana1.16" }`
    - Solana ~1.18: `solana-test-framework = { git = "https://github.com/halbornlabs/solana-test-framework", branch = "solana1.18" }`

2. include `features = ["anchor"]` in your dependency declaration if you want to enable Anchor convenience methods

&nbsp;

## Docs
### [`BanksClient`](https://docs.rs/solana-banks-client/latest/solana_banks_client/struct.BanksClient.html) and [`RpcClient`](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html) extensions

Assemble the given instructions into a transaction and sign it.
All transactions created with this method are signed and payed for by the payer.

```rust
async fn transaction_from_instructions(
    &mut self,
    ixs: &[Instruction],
    payer: &Keypair,
    signers: Vec<&Keypair>
) -> Result<Transaction, Box<dyn std::error::Error>>
```

&nbsp;

Return and deserialize an [`Anchor`](https://docs.rs/anchor-lang/latest/anchor_lang/trait.AccountDeserialize.html) account at the given address at the time of the most recent root slot.
If the account is not found, `None` is returned.

```rust
#[cfg(feature = "anchor")]
async fn get_account_with_anchor<T: AccountDeserialize>(
    &mut self,
    address: Pubkey
) -> Result<T, Box<dyn std::error::Error>>
```

&nbsp;

Return and deserialize a [`Borsh`](https://docs.rs/borsh/latest/borsh/) account at the given address at the time of the most recent root slot.
If the account is not found, `None` is returned.

```rust
async fn get_account_with_borsh<T: BorshDeserialize>(
    &mut self,
    address: Pubkey
) -> Result<T, Box<dyn std::error::Error>>
```

&nbsp;

Create a new account.

```rust
async fn create_account(
    &mut self,
    from: &Keypair,
    to: &Keypair,
    lamports: u64,
    space: u64,
    owner: Pubkey
) -> Result<(), Box<dyn std::error::Error>>
```

&nbsp;

Create a new SPL Token [`Mint`](https://docs.rs/spl-token/latest/spl_token/state/struct.Mint.html) account.

```rust
async fn create_token_mint(
    &mut self,
    mint: &Keypair,
    authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
    payer: &Keypair
) -> Result<(), Box<dyn std::error::Error>>
```

&nbsp;

Create a new SPL Token [`Account`](https://docs.rs/spl-token/latest/spl_token/state/struct.Account.html).

```rust
async fn create_token_account(
    &mut self,
    account: &Keypair,
    authority: &Pubkey,
    mint: &Pubkey,
    payer: &Keypair
) -> Result<(), Box<dyn std::error::Error>>
```

&nbsp;

Create a new [SPL Associated Token account](https://spl.solana.com/associated-token-account)

```rust
async fn create_associated_token_account(
    &mut self,
    authority: &Pubkey,
    mint: &Pubkey,
    payer: &Keypair
) -> Result<Pubkey, Box<dyn std::error::Error>>
```

&nbsp;

Deploy a final program

```rust
async fn deploy_program(
    &mut self,
    path_to_program: &str,
    program_keypair: &Keypair,
    payer: &Keypair,
) -> Result<(), Box<dyn std::error::Error>>
```

&nbsp;

Deploy an upgradeable program

```rust
async fn deploy_upgradable_program(
    &mut self,
    _path_to_program: &str,
    _buffer_keypair: &Keypair,
    _buffer_authority_signer: &Keypair,
    _program_keypair: &Keypair,
    _payer: &Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
```

&nbsp;

### [`ProgramTest`](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTest.html) extensions

Add a rent-exempt account with some data to the test environment.

```rust
pub fn add_account_with_data(
    &mut self,
    pubkey: Pubkey,
    owner: Pubkey,
    data: &[u8],
    executable: bool,
)
```

&nbsp;

Add an [`Anchor`](https://docs.rs/anchor-lang/latest/anchor_lang/attr.account.html) account to the test environment.

```rust
#[cfg(feature = "anchor")]
pub fn add_account_with_anchor<T: AccountSerialize + AnchorSerialize + Discriminator>(
    &mut self,
    pubkey: Pubkey,
    owner: Pubkey,
    anchor_data: T,
    executable: bool,
)
```

Add an empty [`Anchor`](https://docs.rs/anchor-lang/latest/anchor_lang/attr.account.html) account to the test environment with a specified data size. Note the total size of the accounts data is 8 (discriminator) + size.
```rust
#[cfg(feature = "anchor")]
pub fn add_empty_account_with_anchor<T: AccountSerialize + AnchorSerialize + Discriminator>(
    &mut self,
    pubkey: Pubkey,
    owner: Pubkey,
    size: u64,
)

local_env_builder.add_empty_account_with_anchor::<HelloCounter>(user_pubkey, program::id(), 32);

```
&nbsp;

Add an account with the given balance to the test environment.

```rust
pub fn add_account_with_lamports(
    &mut self,
    pubkey: Pubkey,
    owner: Pubkey,
    lamports: u64,
)
```

&nbsp;

Add a rent-exempt account with some [`Pack`able](https://docs.rs/solana-program/latest/solana_program/program_pack/trait.Pack.html) data to the test environment.

```rust
pub fn add_account_with_packable<P: Pack>(
    &mut self,
    pubkey: Pubkey,
    owner: Pubkey,
    data: P,
)
```

&nbsp;

Add a rent-exempt account with some [`Borsh`](https://docs.rs/borsh/latest/borsh/)-serializable to the test environment

```rust
pub fn add_account_with_borsh<B: BorshSerialize>(
    &mut self,
    pubkey: Pubkey,
    owner: Pubkey,
    data: B
)
```

&nbsp;

Generate and add multiple accounts to the test environment.

```rust
pub fn generate_accounts(
    &mut self,
    number_of_accounts: u8,
    initial_lamports: u64) -> Vec<Keypair>
```

&nbsp;

Add an SPL Token [`Mint`](https://docs.rs/spl-token/latest/spl_token/state/struct.Mint.html) account to the test environment.

```rust
pub fn add_token_mint(
   &mut self,
   pubkey: Pubkey,
   mint_authority: Option<Pubkey>,
   supply: u64,
   decimals: u8,
   freeze_authority: Option<Pubkey>,
)
```

&nbsp;

Add an SPL Token [`Account`](https://docs.rs/spl-token/latest/spl_token/state/struct.Account.html) to the test environment.

```rust
fn add_token_account(
    &mut self,
    pubkey: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
    delegate: Option<Pubkey>,
    is_native: Option<u64>,
    delegated_amount: u64,
    close_authority: Option<Pubkey>
)
```

&nbsp;

Add an [associated SPL Token account](https://spl.solana.com/associated-token-account) to the test environment.
Returns the address of the created account.

```rust
fn add_associated_token_account(
    &mut self,
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
    delegate: Option<Pubkey>,
    is_native: Option<u64>,
    delegated_amount: u64,
    close_authority: Option<Pubkey>
) -> Pubkey
```

&nbsp;

Add a BPF program to the test environment.
The program is upgradeable if `Some` `program_authority` is provided.

```rust
fn add_bpf_program(
    &mut self,
    program_name: &str,
    program_id: Pubkey,
    program_authority: Option<Pubkey>,
    process_instruction: Option<ProcessInstructionWithContext>
)
```

Adds BPF program to the test environment.
The program is upgradeable if `Some` `program_authority` with the `program data` provided.
This is useful for those programs which the program data has to be a spefic one, if not, use add_bpf_program
```rust
fn add_bpf_program_with_program_data(
    &mut self,
    program_name: &str,
    program_id: Pubkey,
    program_authority: Option<Pubkey>,
    program_data: Pubkey,
    upgrade_slot: u64,
    process_instruction: Option<ProcessInstructionWithContext>,
)
```
&nbsp;

### [`ProgramTestContext`](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTestContext.html) extensions

Advance the internal clock to the provided timestamp.

```rust
async fn warp_to_timestamp(
  &mut self,
  timestamp: i64
) -> Result<(), ProgramTestError>
```

Update the Price Account or Price Info, Time Stamp and Valid Slots of a Pyth Oracle.

```rust
async fn update_pyth_oracle(
    &mut self,
    address: Pubkey,
    price_account: Option<PriceAccount>,
    price_info: Option<PriceInfo>,
    timestamp: Option<i64>,
    valid_slots: Option<u64>,
) -> Result<(), TestFrameWorkError>
```
