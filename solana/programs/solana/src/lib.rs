use anchor_lang::prelude::*;

declare_id!("77KSMNB3WMeW7BqhxYzYChidwcuFiCKZvGGtpG9MgXwV");

#[program]
pub mod openbadges {
    use super::*;

    pub fn create_badge(ctx: Context<CreateBadge>, badge: Badge) -> Result<()>  {
        let badge_account = &mut ctx.accounts.badge;

        badge_account.context = badge.context;
        badge_account.badge_id = badge_account.key();
        badge_account.badge_type = badge.badge_type;
        badge_account.issuer = badge.issuer;
        badge_account.issuance_date = badge.issuance_date;
        badge_account.owner = badge.owner;
        badge_account.name = badge.name;
        badge_account.credential_subject = badge.credential_subject;
        Ok(())
    }


    pub fn verify(ctx: Context<Verify>) -> Result<()> {
        let badge_account = &mut ctx.accounts.badge;
        let owner = &mut ctx.accounts.owner;
        
        // Check if the issuer (who would be the caller in this case) is the badge owner
        if badge_account.owner != *owner.to_account_info().key.to_string() {
            return err!(ErrorCode::NotValid);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Verify<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub badge: Account<'info, BadgeAccount>,
}

#[derive(Accounts)]
pub struct CreateBadge<'info> {
    #[account(init, payer = issuer, space = 8 + 1024)]
    pub badge: Account<'info, BadgeAccount>,
    #[account(mut)]
    pub issuer: Signer<'info>,
    //better approach but, not working
    //pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BadgeAccount {
    pub context: Vec<String>,
    pub badge_id: Pubkey,
    pub badge_type: Vec<String>,
    pub issuer: Issuer,
    pub issuance_date: String,
    pub owner: String,
    pub name: String,
    pub credential_subject: CredentialSubject,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Badge {
    pub context: Vec<String>,
    pub badge_id: String,
    pub badge_type: Vec<String>,
    pub issuer: Issuer,
    pub issuance_date: String,
    pub owner: String,
    pub name: String,
    pub credential_subject: CredentialSubject,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Issuer {
    pub id: String,
    pub issuer_type: String,
    pub name: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CredentialSubject {
    pub id: String,
    pub subject_type: String,
    pub achievement: Achievement,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Achievement {
    pub id: String,
    pub achievement_type: String,
    pub criteria: Criteria,
    pub description: String,
    pub name: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Criteria {
    pub narrative: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not valid")]
    NotValid,
}