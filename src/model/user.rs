use bitflags::bitflags;
use serde::Deserialize;

use super::snowflake::Snowflake;

bitflags! {
    #[derive(Deserialize)]
    pub struct UserFlags: u32 {
        const NONE = 0;
        /// Discord Employee
        const STAFF = 1 << 0;
        /// Partnered Server Owner
        const PARTNER = 1 << 1;
        /// HypeSquad Events Coordinator
        const HYPESQUAD = 1 << 2;
        /// Bug Hunter Level 1
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        /// House Bravery Member
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        /// House Brilliance Member
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        /// House Balance Member
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        /// Early Nitro Supporter
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;
        /// User is a team
        const TEAM_PSEUDO_USER = 1 << 10;
        /// Bug Hunter Level 2
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        /// Verified Bot
        const VERIFIED_BOT = 1 << 16;
        /// Early Verified Bot Developer
        const VERIFIED_DEVELOPER = 1 << 17;
        /// Discord Certified Moderator
        const CERTIFIED_MODERATOR = 1 << 18;
        /// Bot uses only HTTP interactions and is shown in the online member list
        const BOT_HTTP_INTERACTIONS = 1 << 19;
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum PremiumType {
    None = 1,
    NitroClassic,
    Nitro,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    /// The user's ID.
    pub id: Snowflake,
    /// The user's username.
    pub username: String,
    /// The user's discriminator.
    pub discriminator: String,
    /// The user's avatar hash.
    pub avatar: Option<String>,
    /// Whether this user belongs to an OAuth-2 application.
    pub bot: bool,
    /// Whether the user is an Official Discord System user (part of the urgent message system)
    pub system: bool,
    /// Whether the user has 2FA enabled on their account.
    pub mfa_enabled: bool,
    /// The user's banner hash.
    pub banner: Option<String>,
    /// The user's banner color encoded as an integer representation of hexadecimal color code.
    pub accent_color: Option<u32>,
    /// The user's chosen language option.
    pub locale: String,
    /// The flags on a user's account.
    pub flags: UserFlags,
    /// The user's public flags.
    pub public_flags: UserFlags,
    /// The type of Nitro subscription on a user's account.
    pub premium_type: PremiumType,
}

impl User {
    pub fn tag(&self) -> String {
        return format!("{}#{}", self.username, self.discriminator);
    }
}
