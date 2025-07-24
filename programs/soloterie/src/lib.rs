use anchor_lang::prelude::*;

declare_id!("5ieKhngV4QqQZBhrzKbt11dU9y1hRrKomioHHGEB9tR5");

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

    pub fn buy_ticket(ctx: Context<BuyTicket>, number_of_tickets: u64) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        let user = &ctx.accounts.user;
    
        // Vérifie que le nombre demandé est > 0
        require!(number_of_tickets > 0, LotteryError::InvalidTicketNumber);
    
        // Vérifie qu’il reste assez de tickets
        require!(
            lottery.tickets_sold + number_of_tickets <= lottery.nb_tickets,
            LotteryError::LotteryFull
        );
    
        // Calcul du prix total
        let total_price = lottery.ticket_price.checked_mul(number_of_tickets)
            .ok_or(LotteryError::CalculationError)?;
    
        // Transfert des SOL
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: user.to_account_info(),
                to: lottery.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_ctx, total_price)?;
    
        // Incrémente le compteur de tickets vendus
        lottery.tickets_sold += number_of_tickets;
    
        msg!("{} tickets achetés ! Total vendus : {}", number_of_tickets, lottery.tickets_sold);
    
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

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}



#[error_code]
pub enum LotteryError {
    #[msg("Le prix du ticket doit être supérieur à zéro.")]
    InvalidTicketPrice,
    
    #[msg("Le max SOL doit être >= au prix du ticket.")]
    InvalidMaxSol,
    
    #[msg("La loterie est complète, tous les tickets ont été vendus.")]
    LotteryFull,

    #[msg("Le nombre de ticket demandé est incorrect")]
    InvalidTicketNumber,

    #[msg("Erreur lors du calcul du prix de la transaction")]
    CalculationError,
    
    
}

