use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};
use solana_program::blake3::hash;

// input bet amount, choose hand (1-5), make guess (0-10)

declare_id!("8ce52PpcApfmiNVX45tFrudt9kHYJQXSJuC8ZzxUxhMo");

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

#[program]
pub mod morra_lotto {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        hash: [u8; 32],
        bet_amount: u64,
        hand: u8,
        guess: u8,
    ) -> Result<()> {
        assert!(
            guess < 11 && hand < 6,
            "guess must be less than 11 & hand less than 6"
        );
        // init GameState

        let game = &mut ctx.accounts.game_state;
        game.player1 = *ctx.accounts.player1.key;
        game.player2 = None;
        // game.hash1 = Pubkey::default();
        // game.hash2 = Pubkey::default();
        game.player1_hand = hand;
        game.player2_hand = 0;

        game.game_round = 0;
        game.bet_amount = bet_amount;
        // game.player1_last_round = *ctx.accounts;

        // player1_last_round: u16,
        // player2_last_round: u16,

        // startedAt: i64,
        // timeout: i64,

        game.bet_amount = bet_amount;
        game.player1_guess = guess;
        game.player1_guess = 0;

        // init VaultState
        let vault_state = &mut ctx.accounts.vault_state;
        vault_state.owner = *ctx.accounts.player1.key;
        vault_state.auth_bump = *ctx.bumps.get("vault_auth").unwrap();
        vault_state.vault_bump = *ctx.bumps.get("vault").unwrap();

        // Transfer bet amount to vault

        let accounts = Transfer {
            from: ctx.accounts.player1.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let context = CpiContext::new(ctx.accounts.system_program.to_account_info(), accounts);
        system_program::transfer(context, bet_amount)

        // Ok(())
    }

    pub fn play(ctx: Context<Play>, hand: u8) -> Result<()> {
        assert!(hand < 6);

        let mut game_state = ctx.accounts.seed.key().to_bytes().to_vec();
        game_state.extend_from_slice(&[hand]);
        let converted_seed: u8 = game_state.iter().sum();
        let hash = hash_stuff(converted_seed);

        let game = &ctx.accounts.game;

        // require_eq!(game, game.hash);

        let win = (game.hand1 + hand) == game.guess1;

        if win {
            let payout = game.bet_amount * 2;
            let cpi_program = ctx.accounts.system_program.to_account_info();
            let cpi_accounts = anchor_lang::system_program::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.player1.to_account_info(),
            };

            let seeds = &[
                "vault".as_bytes(),
                &ctx.accounts.vault_auth.key().clone().to_bytes(),
                &[ctx.accounts.vault_state.vault_bump],
            ];

            let signer_seeds = &[&seeds[..]];
            let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            anchor_lang::system_program::transfer(cpi_context, payout)?;
        }
        Ok(())
    }
}

pub fn hash_stuff(hand: u8) -> [u8; 32] {
    let to_hash = vec![hand];
    hash(to_hash.as_slice()).to_bytes()
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub player1: Signer<'info>,
    #[account(init, payer = player1, space = 8 + 32 + 3)]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    ///CHECK: NO NEED TO CHECK THIS
    pub vault_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        seeds = [b"state".as_ref(), player1.key().as_ref()],
        bump,
        payer = player1,
        space = Game::LEN
    )]

    pub game_state: Account<'info, Game>,
    // #[account(init, payer = player1, space = Game::LEN)]
    // pub game: Account<'info, Game>,
    /// CHECK: NO NEED TO CHECK THIS
    pub hash: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub player1: Signer<'info>,
    #[account(init, payer = player1, space = 8 + 32 + 3)]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    ///CHECK: NO NEED TO CHECK THIS
    pub vault_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,
    #[account(mut, seeds = [b"hash", player1.key().as_ref()], bump)]
    ///CHECK: NO NEED TO CHECK THIS
    // pub game_hash: UncheckedAccount<'info>,
    #[account(has_one = player1)]
    pub game: Account<'info, Game>,
    /// CHECK: NO NEED TO CHECK THIS
    #[account(mut)]
    pub seed: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct RevealMoves <'info> {
//     #[account(mut)]
//     pub player: Signer<'info>,
//     #[account(mut, seeds = [b"ticket", player.key().as_ref()], bump)]
//        ///CHECK: NO NEED TO CHECK THIS
//     pub game_hash: UncheckedAccount<'info>,
//     pub ticket_info: Account<'info, TicketInfo>,
//     pub game_state: Account<'info, Game>,
//     pub system_program: Program<'info, System>,
// }

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
    hash: Pubkey,
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

// #[account]
// pub struct GameState {
//     total_tickets: u64,
//     ticket_price: u64,
//     min_move: u8,
//     max_move: u8,
//     stage_link: u64,
//     total_sum: u64,

//     // base_pot: u16
// }
#[account]
pub struct Game {
    player1: Pubkey,

    hash1: Pubkey,
    hash2: Pubkey,

    player1_hand: u8,
    player2_hand: u8,

    player1_guess: u8,
    player2_guess: u8,

    game_round: u16,

    player1_last_round: u16,
    player2_last_round: u16,

    bet_amount: u64,
    startedAt: i64,
    timeout: i64,

    player2: Option<Pubkey>,
}

impl Game {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 1 + 1;
}
