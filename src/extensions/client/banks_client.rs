use super::*;

#[async_trait]
impl ClientExtensions for BanksClient {
    async fn transaction_from_instructions(
        &mut self,
        ixs: &[Instruction],
        payer: &Keypair,
        signers: Vec<&Keypair>,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let latest_blockhash = self.get_latest_blockhash().await?;

        Ok(Transaction::new_signed_with_payer(
            ixs,
            Some(&payer.pubkey()),
            &signers,
            latest_blockhash,
        ))
    }

    #[cfg(feature = "anchor")]
    async fn get_account_with_anchor<T: AccountDeserialize>(
        &mut self,
        address: Pubkey,
    ) -> Result<T, Box<dyn std::error::Error>> {
        self.get_account(address).map(|result| {
            let account = result?.ok_or(BanksClientError::ClientError("Account not found"))?;
            T::try_deserialize(&mut account.data.as_ref()).map_err(|_| BanksClientError::ClientError("Failed to deserialize account"))
        }).await.map_err(Into::into)
    }

    async fn get_account_with_borsh<T: BorshDeserialize>(
        &mut self,
        address: Pubkey,
    ) -> Result<T, Box<dyn std::error::Error>> {
        self.get_account(address).map(|result| {
            let account = result?.ok_or(BanksClientError::ClientError("Account not found"))?;
            T::deserialize(&mut account.data.as_ref()).map_err(|_| BanksClientError::ClientError("Failed to deserialize account"))
        }).await.map_err(Into::into)
    }

    async fn create_account(
        &mut self,
        from: &Keypair,
        to: &Keypair,
        lamports: u64,
        space: u64,
        owner: Pubkey,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let latest_blockhash = self.get_latest_blockhash().await?;

        self.process_transaction(system_transaction::create_account(
            from,
            to,
            latest_blockhash,
            lamports,
            space,
            &owner,
        ))
        .await
        .map_err(Into::into)
    }

    async fn create_token_mint(
        &mut self,
        mint: &Keypair,
        authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
        decimals: u8,
        payer: &Keypair,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let latest_blockhash = self.get_latest_blockhash().await?;
        self.process_transaction(system_transaction::create_account(
            &payer,
            &mint,
            latest_blockhash,
            Rent::default().minimum_balance(spl_token::state::Mint::get_packed_len()),
            spl_token::state::Mint::get_packed_len() as u64,
            &spl_token::id(),
        ))
        .await
        .unwrap();

        let ix = spl_token::instruction::initialize_mint(
            &spl_token::id(),
            &mint.pubkey(),
            authority,
            freeze_authority,
            decimals,
        )
        .unwrap();

        self.process_transaction(Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[payer],
            latest_blockhash,
        ))
        .await
        .map_err(Into::into)
    }

    async fn create_token_account(
        &mut self,
        account: &Keypair,
        authority: &Pubkey,
        mint: &Pubkey,
        payer: &Keypair,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let latest_blockhash = self.get_latest_blockhash().await?;
        self.process_transaction(system_transaction::create_account(
            &payer,
            &account,
            latest_blockhash,
            Rent::default().minimum_balance(spl_token::state::Account::get_packed_len()),
            spl_token::state::Account::get_packed_len() as u64,
            &spl_token::id(),
        ))
        .await
        .unwrap();

        let ix = spl_token::instruction::initialize_account(
            &spl_token::id(),
            &account.pubkey(),
            mint,
            authority,
        )
        .unwrap();

        self.process_transaction(Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[payer],
            latest_blockhash,
        ))
        .await
        .map_err(Into::into)
    }

    async fn create_associated_token_account(
        &mut self,
        account: &Pubkey,
        mint: &Pubkey,
        payer: &Keypair,
        token_program_id: &Pubkey,
    ) -> Result<Pubkey, Box<dyn std::error::Error>> {
        let latest_blockhash = self.get_latest_blockhash().await?;
        let associated_token_account = get_associated_token_address(
            account,
            mint
        );
        let ix = create_associated_token_account_ix(
        &payer.pubkey(),
        &account,
        &mint,
        &token_program_id
        );

        self.process_transaction(Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[payer],
            latest_blockhash,
        ))
        .await?;

        return Ok(associated_token_account);
    }

    async fn deploy_program(
        &mut self,
        path_to_program: &str,
        program_keypair: &Keypair,
        payer: &Keypair,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (buffer, buffer_len) = util::load_file_to_bytes(path_to_program);

        let program_data = buffer;

        // multiply by 2 so program can be updated later on
        let program_len = buffer_len;
        let minimum_balance = Rent::default().minimum_balance(
            bpf_loader_upgradeable::UpgradeableLoaderState::programdata_len(program_len)
                .expect("Cannot get program len"),
        );
        let latest_blockhash = self.get_latest_blockhash().await?;

        // 1 Create account
        self.process_transaction(system_transaction::create_account(
            &payer,
            &program_keypair,
            latest_blockhash,
            minimum_balance,
            program_len as u64,
            &bpf_loader::id(),
        ))
        .await
        .unwrap();

        // 2. Write to buffer
        let deploy_ix = |offset: u32, bytes: Vec<u8>| {
            loader_instruction::write(&program_keypair.pubkey(), &bpf_loader::id(), offset, bytes)
        };

        let chunk_size = util::calculate_chunk_size(&deploy_ix, &vec![payer, program_keypair]);

        for (chunk, i) in program_data.chunks(chunk_size).zip(0..) {
            let ix = deploy_ix(i * chunk_size as u32, chunk.to_vec());
            let tx = self
                .transaction_from_instructions(&[ix], &payer, vec![&payer, &program_keypair])
                .await
                .unwrap();

            self.process_transaction(tx).await?;
        }

        // 3. Finalize
        let finalize_tx = self.transaction_from_instructions(
            &[
                loader_instruction::finalize(
                    &program_keypair.pubkey(),
                    &bpf_loader::id(),
                )   
            ],
            &payer,
            vec![
                payer,
                program_keypair
            ]
        ).await.unwrap();

        self.process_transaction(finalize_tx).await?;

        return Ok(());
    }

    async fn deploy_upgradable_program(
        &mut self,
        path_to_program: &str,
        buffer_keypair: &Keypair,
        buffer_authority_signer: &Keypair,
        program_keypair: &Keypair,
        payer: &Keypair,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (buffer, buffer_len) = util::load_file_to_bytes(path_to_program);

        let program_data = buffer;

        // multiply by 2 so program can be updated later on
        let program_len = buffer_len * 2;
        let minimum_balance = Rent::default().minimum_balance(
            bpf_loader_upgradeable::UpgradeableLoaderState::programdata_len(program_len)
                .expect("Cannot get program len"),
        );

        // 1 Create buffer
        let create_buffer_ix = bpf_loader_upgradeable::create_buffer(
            &payer.pubkey(),
            &buffer_keypair.pubkey(),
            &buffer_authority_signer.pubkey(),
            minimum_balance,
            program_len,
        )
        .expect("Cannot create buffer");

        let mut tx = self
            .transaction_from_instructions(
                create_buffer_ix.as_ref(),
                &payer,
                vec![&payer, &buffer_keypair],
            )
            .await
            .unwrap();

        self.process_transaction(tx).await?;

        // 2 Write to buffer
        let deploy_ix = |offset: u32, bytes: Vec<u8>| {
            bpf_loader_upgradeable::write(
                &buffer_keypair.pubkey(),
                &buffer_authority_signer.pubkey(),
                offset,
                bytes,
            )
        };

        let chunk_size =
            util::calculate_chunk_size(&deploy_ix, &vec![payer, buffer_authority_signer]);

        for (chunk, i) in program_data.chunks(chunk_size).zip(0..) {
            let ix = deploy_ix(i * chunk_size as u32, chunk.to_vec());

            tx = self
                .transaction_from_instructions(
                    &[ix],
                    &payer,
                    vec![&payer, &buffer_authority_signer],
                )
                .await
                .unwrap();

            self.process_transaction(tx).await?;
        }

        // 3. Finalize
        tx = self
            .transaction_from_instructions(
                bpf_loader_upgradeable::deploy_with_max_program_len(
                    &payer.pubkey(),
                    &program_keypair.pubkey(),
                    &buffer_keypair.pubkey(),
                    &buffer_authority_signer.pubkey(),
                    minimum_balance,
                    program_len,
                )
                .expect("Cannot parse deploy instruction")
                .as_ref(),
                &payer,
                vec![&payer, &program_keypair, &buffer_authority_signer],
            )
            .await
            .unwrap();
        self.process_transaction(tx).await?;

        return Ok(());
    }
}