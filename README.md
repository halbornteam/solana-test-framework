# solana-test-framework

`solana-test-framework` is build on top of the [`solana-program-test`](https://docs.rs/crate/solana-program-test/latest) crate and it provides a [`BanksClient`](https://docs.rs/solana-banks-client/latest/solana_banks_client/struct.BanksClient.html)-based Proof of Concept framework for BPF programs.
It extends [`BanksClient`](https://docs.rs/solana-banks-client/latest/solana_banks_client/struct.BanksClient.html),
[`ProgramTest`](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTest.html)
and [`ProgramTestContext`](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTestContext.html) with several convenience methods.

&nbsp;
&nbsp;

## Setup
This framework supports Solana v1.9 and Anchor v0.24.2 **OR** Solana v1.10 and Anchor v0.25.0. To use it in your project,

1. add one of the following in your `Cargo.toml`:
 
    - Solana ~1.9: `solana-test-framework = { git = "https://github.com/lowprivuser/solana-test-framework"}`
    - Solana ~1.10: `solana-test-framework = { git = "https://github.com/lowprivuser/solana-test-framework", branch = "solana1.10" }`

2. include `features = ["anchor"]` in your dependency declaration if you want to enable Anchor convenience methods

&nbsp;

## Docs
### [`BanksClient`](https://docs.rs/solana-banks-client/latest/solana_banks_client/struct.BanksClient.html) extensions

Assemble the given instructions into a transaction and sign it.
All transactions created with this method are signed and payed for by the payer.

```rust
async fn transaction_from_instructions(
    &mut self,
    ixs: &[Instruction],
    payer: &Keypair,
    signers: Vec<&Keypair>
) -> Result<Transaction, BanksClientError>
```

&nbsp;

Return and deserialize an [`Anchor`](https://docs.rs/anchor-lang/latest/anchor_lang/trait.AccountDeserialize.html) account at the given address at the time of the most recent root slot.
If the account is not found, `None` is returned.

```rust
#[cfg(feature = "anchor")]
fn get_account_with_anchor<T: AccountDeserialize>(
    &mut self,
    address: Pubkey
) -> Pin<Box<dyn Future<Output = Result<T, BanksClientError>> + '_>>
```

&nbsp;

Return and deserialize a [`Borsh`](https://docs.rs/borsh/latest/borsh/) account at the given address at the time of the most recent root slot.
If the account is not found, `None` is returned.

```rust
fn get_account_with_borsh<T: BorshDeserialize>(
    &mut self,
    address: Pubkey
) -> Pin<Box<dyn Future<Output = Result<T, BanksClientError>> + '_>>
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
) -> transport::Result<Pubkey> {
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
) -> transport::Result<Pubkey> {
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
) -> transport::Result<Pubkey> {
```

&nbsp;

Create a new [SPL Associated Token account](https://spl.solana.com/associated-token-account)

```rust
async fn create_associated_token_account(
    &mut self,
    authority: &Pubkey,
    mint: &Pubkey,
    payer: &Keypair
) -> transport::Result<Pubkey> {
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

### [`ProgramTestContext`](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTestContext.html) extensions

Advance the internal clock to the provided timestamp.

```rust
async fn warp_to_timestamp(
  &mut self,
  timestamp: i64
) -> Result<(), ProgramTestError>
```

&nbsp;

Deploy program

```rust
async fn deploy_program(
    &mut self,
    path_to_program: &str,
    program_keypair: &Keypair,
    payer: &Keypair,
) -> transport::Result<()>
```

&nbsp;

Deploy upgradable program

```rust
async fn deploy_upgradable_program(
    &mut self,
    path_to_program: &str,
    buffer_keypair: &Keypair,
    buffer_authority_signer: &Keypair,
    program_keypair: &Keypair,
    payer: &Keypair,
) -> transport::Result<()>
```

&nbsp;
