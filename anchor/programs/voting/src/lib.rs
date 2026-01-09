use anchor_lang::prelude::*;



declare_id!("9LhAwg9BkbRpdSnRmWJTexVxBQoPrybZ3ee5WMH1kncr");

#[program]
pub mod voting {
    use super::*;
    pub fn initialize_poll(_ctx:Context<IntializePoll>,_poll_id:u64)->Result<()>{
        Ok(())
    }
}   

#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct IntializePoll<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(
        init,
        payer=signer,
        space=8+Poll::INIT_SPACE,
        seeds=[poll_id.to_le_bytes().as_ref()], 
        bump

    )]
    pub poll:Account<'info,Poll>,
    pub system_program:Program<'info,System>,

}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id:u64,
    #[max_len(280)]
    pub description:String,
    pub poll_start:u64,
    pub poll_end: u64,
    pub candidate_amount:u64,

}