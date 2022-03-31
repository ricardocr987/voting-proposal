use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod voting_proposal {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>, 
        proposal_bump: u8,
        end_timestamp: i64,
    ) -> Result<()> {

        let proposal = &mut *ctx.accounts.proposal_account;
        
        proposal.proposer = *ctx.accounts.proposer.to_account_info().key;
        proposal.pro_count = 0;
        proposal.against_count = 0;
        proposal.end_timestamp = end_timestamp;
        proposal.proposal_bump = proposal_bump;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = proposer,
        space = 8 + 32 + 16 + 16 + 8,
        seeds = [
            proposer.key().as_ref(), // with this seeds we avoid that a proposer make multiple PDA's, can't be two PDA 
                                    // Proposal Account actives at the same time
        ],
        bump
    )]
    pub proposal_account: Account<'info, ProposalAccount>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProposalAccount{
    pub proposer: Pubkey,
    pub pro_count: u128,
    pub against_count: u128,
    pub end_timestamp: i64,
    pub proposal_bump: u8
}

