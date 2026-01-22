use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Copy)]
pub enum RaffleInstruction {
    EnterRaffle,
    DrawWinner,
}

#[account]
pub struct RaffleState {
    pub admin: Pubkey,
    pub participants: Vec<Pubkey>,
    pub pot: u64,  // Total SOL collected
}

impl Space for RaffleState {
    const INIT_SPACE: usize = 8 + 32 + 1024 * 32; // 32B for admin, ~3KB for participants list
}

#[program]
mod raffle {
    use super::*;

    pub fn enter_raffle(ctx: Context<EnterRaffle>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let participant = ctx.accounts.participant.key();
        
        // Add participant to the list
        if !state.participants.contains(&participant) {
            state.participants.push(participant);
            state.pot += ctx.accounts.spl_token_transfer_lamports?;
        }

        Ok(())
    }

    pub fn draw_winner(ctx: Context<DrawWinner>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let winner_index = if !state.participants.is_empty() {
            (ctx.accounts.winner.key().as_ref()[0] % state.participants.len() as u8) as usize
        } else {
            return Err(ProgramError::InvalidArgument.into());
        };

        let winner = &state.participants[winner_index];
        
        // Transfer SOL to the winner
        **ctx.accounts.winner.try_borrow_mut_lamports()? += *ctx.accounts.state.pot;
        **ctx.accounts.state.to_account_info().try_borrow_mut_lamports()? -= state.pot;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnterRaffle<'info> {
    #[account(mut)]
    pub participant: Signer<'info>,
    #[account(init, payer = participant, space = RaffleState::INIT_SPACE)]
    pub state: Account<'info, RaffleState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DrawWinner<'info> {
    #[account(mut)]
    pub winner: Signer<'info>,
    #[account(mut)]
    pub state: Account<'info, RaffleState>,
}
