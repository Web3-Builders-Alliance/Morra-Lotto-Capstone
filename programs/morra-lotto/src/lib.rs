use anchor_lang::prelude::*;
use anchor_lang::{
    system_program::{self, Transfer},
};
use solana_program::blake3::hash;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;


#[program]
pub mod morra_lotto {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // init GameState
        let game_state = &mut ctx.accounts.game_state;
        game_state.total_tickets = 0;
        game_state.ticket_price = 1 * LAMPORTS_PER_SOL;
        game_state.min_move = 0;
        game_state.max_move = 5;


        // init VaultState
        let vault_state = &mut ctx.accounts.vault_state;
        vault_state.owner = *ctx.accounts.buyer.key;
        vault_state.auth_bump = *ctx.bumps.get("vault_auth").unwrap();
        vault_state.vault_bump = *ctx.bumps.get("vault").unwrap();


        // game_state.stage_link = 
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, hash: [u8;32] ) -> Result<()> {

        // init player
        ctx.accounts.ticket_state.player = ctx.accounts.buyer.key();

        // pass in hash of hand and sum. (have to save in state)

        // let hashed = &mut ctx.accounts.ticket_state;


    let game_state = &mut ctx.accounts.game_state;

    game_state.ticket_price = 1 * LAMPORTS_PER_SOL;

    let accounts = Transfer {
    from: ctx.accounts.buyer.to_account_info(),
    to: ctx.accounts.vault.to_account_info(),
    };

    let context =
    CpiContext::<Transfer>::new(ctx.accounts.system_program.to_account_info(), accounts);
    system_program::transfer(context, game_state.ticket_price)

    }

    // pub fn reveal_moves(ctx: Context<RevealMoves>) -> Result<()> {

    //     Ok(())
    // }

    // pub fn claim(ctx: Context<Claim>) -> Result<()> {

    //     Ok(())
    // }


}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(init, payer = buyer, space = 8 + 32 + 3)]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    ///CHECK: NO NEED TO CHECK THIS
    pub vault_auth:  UncheckedAccount<'info,>,
    #[account(mut, seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        seeds = [b"state".as_ref(), buyer.key().as_ref()],
        bump,
        payer = buyer,
        space = 8 + 8 + 8 + 8 + 1 + 1,
    )]
    pub game_state: Account<'info, GameState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(hash: [u8;32])]
pub struct BuyTicket <'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(init, payer = buyer, space = 8 + 32 + 3)]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    ///CHECK: NO NEED TO CHECK THIS
    pub vault_auth:  UncheckedAccount<'info,>,
    #[account(mut, seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,
    #[account(mut, seeds = [b"ticket", buyer.key().as_ref()], bump)]
    pub ticket_state: Account<'info, TicketInfo>,
    pub game_state: Account<'info, GameState>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    owner: Pubkey,
    auth_bump: u8,
    vault_bump: u8,
}

#[account]
pub struct TicketInfo {
    player: Pubkey,
    player_move: u8,
    guess_sum: u8,
    hash: [u8; 32],
}

impl TicketInfo {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 32;
}

// impl TicketInfo {
//     pub fn hash_players_inputs(&self, player_move: u8, player_guess: u32) {
//         // players move
//          let mut to_hash = player_move.to_le_bytes().to_vec();
//         // players guess of the sum
//         to_hash.extend_from_slice(player_guess.to_le_bytes().as_ref());

//         to_hash.extend_from_slice(b"secret");

//         hash(to_hash.as_slice());

//     }
// }

#[account]
pub struct GameState {
    total_tickets: u64,
    ticket_price: u64,
    min_move: u8,
    max_move: u8,
    stage_link: u64,
    // base_pot: u16
}

