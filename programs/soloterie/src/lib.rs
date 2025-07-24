use anchor_lang::prelude::*;

declare_id!("5ieKhngV4QqQZBhrzKbt11dU9y1hRrKomioHHGEB9tR5"); // ⚠️ À remplacer après le `anchor build`

#[program]
pub mod soloterie {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, max_sol: u64, ticket_price: u64) -> Result<()> {
        require!(ticket_price > 0, LotteryError::InvalidTicketPrice);
        require!(max_sol >= ticket_price, LotteryError::InvalidMaxSol);

        let lottery = &mut ctx.accounts.lottery;

        lottery.max_sol = max_sol;
        lottery.ticket_price = ticket_price;
        lottery.nb_tickets = max_sol / ticket_price;
        lottery.tickets_sold = 0;

        msg!("Loterie initialisée avec {} tickets.", lottery.nb_tickets);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8*4)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Lottery {
    pub max_sol: u64,
    pub ticket_price: u64,
    pub nb_tickets: u64,
    pub tickets_sold: u64,
}

#[error_code]
pub enum LotteryError {
    #[msg("Le prix du ticket doit être supérieur à zéro.")]
    InvalidTicketPrice,
    #[msg("Le max SOL doit être >= au prix du ticket.")]
    InvalidMaxSol,
}
