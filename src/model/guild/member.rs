use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};
use super::deserialize_sync_user;
use ::model::*;

#[cfg(feature="cache")]
use ::client::{CACHE, rest};
#[cfg(feature="cache")]
use ::internal::prelude::*;
#[cfg(feature="cache")]
use ::utils::builder::EditMember;
#[cfg(feature="cache")]
use ::utils::Colour;

/// Information about a member of a guild.
#[derive(Clone, Debug, Deserialize)]
pub struct Member {
    /// Indicator of whether the member can hear in voice channels.
    pub deaf: bool,
    /// The unique Id of the guild that the member is a part of.
    pub guild_id: Option<GuildId>,
    /// Timestamp representing the date when the member joined.
    pub joined_at: String,
    /// Indicator of whether the member can speak in voice channels.
    pub mute: bool,
    /// The member's nickname, if present.
    ///
    /// Can't be longer than 32 characters.
    pub nick: Option<String>,
    /// Vector of Ids of [`Role`]s given to the member.
    pub roles: Vec<RoleId>,
    /// Attached User struct.
    #[serde(deserialize_with="deserialize_sync_user")]
    pub user: Arc<RwLock<User>>,
}

impl Member {
    /// Adds a [`Role`] to the member, editing its roles in-place if the request
    /// was successful.
    ///
    /// **Note**: Requires the [Manage Roles] permission.
    ///
    /// [`Role`]: struct.Role.html
    /// [Manage Roles]: permissions/constant.MANAGE_ROLES.html
    #[cfg(feature="cache")]
    pub fn add_role<R: Into<RoleId>>(&mut self, role_id: R) -> Result<()> {
        let role_id = role_id.into();

        if self.roles.contains(&role_id) {
            return Ok(());
        }

        let guild_id = self.find_guild()?;

        match rest::add_member_role(guild_id.0, self.user.read().unwrap().id.0, role_id.0) {
            Ok(()) => {
                self.roles.push(role_id);

                Ok(())
            },
            Err(why) => Err(why),
        }
    }

    /// Adds one or multiple [`Role`]s to the member, editing
    /// its roles in-place if the request was successful.
    ///
    /// **Note**: Requires the [Manage Roles] permission.
    ///
    /// [`Role`]: struct.Role.html
    /// [Manage Roles]: permissions/constant.MANAGE_ROLES.html
    #[cfg(feature="cache")]
    pub fn add_roles(&mut self, role_ids: &[RoleId]) -> Result<()> {
        let guild_id = self.find_guild()?;
        self.roles.extend_from_slice(role_ids);

        let map = EditMember::default().roles(&self.roles).0;

        match rest::edit_member(guild_id.0, self.user.read().unwrap().id.0, &map) {
            Ok(()) => Ok(()),
            Err(why) => {
                self.roles.retain(|r| !role_ids.contains(r));

                Err(why)
            }
        }
    }

    /// Ban the member from its guild, deleting the last X number of
    /// days' worth of messages.
    ///
    /// **Note**: Requires the [Ban Members] role.
    ///
    /// # Errors
    ///
    /// Returns a [`ClientError::GuildNotFound`] if the guild could not be
    /// found.
    ///
    /// [`ClientError::GuildNotFound`]: ../client/enum.ClientError.html#variant.GuildNotFound
    ///
    /// [Ban Members]: permissions/constant.BAN_MEMBERS.html
    #[cfg(feature="cache")]
    pub fn ban(&self, delete_message_days: u8) -> Result<()> {
        rest::ban_user(self.find_guild()?.0, self.user.read().unwrap().id.0, delete_message_days)
    }

    /// Determines the member's colour.
    #[cfg(feature="cache")]
    pub fn colour(&self) -> Option<Colour> {
        let guild_id = match self.find_guild() {
            Ok(guild_id) => guild_id,
            Err(_) => return None,
        };

        let cache = CACHE.read().unwrap();
        let guild = match cache.guilds.get(&guild_id) {
            Some(guild) => guild.read().unwrap(),
            None => return None,
        };

        let mut roles = self.roles
            .iter()
            .filter_map(|role_id| guild.roles.get(role_id))
            .collect::<Vec<&Role>>();
        roles.sort_by(|a, b| b.cmp(a));

        let default = Colour::default();

        roles.iter().find(|r| r.colour.0 != default.0).map(|r| r.colour)
    }

    /// Calculates the member's display name.
    ///
    /// The nickname takes priority over the member's username if it exists.
    #[inline]
    pub fn display_name(&self) -> Cow<String> {
        self.nick.as_ref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(self.user.read().unwrap().name.clone()))
    }

    /// Returns the DiscordTag of a Member, taking possible nickname into account.
    #[inline]
    pub fn distinct(&self) -> String {
        format!("{}#{}", self.display_name(), self.user.read().unwrap().discriminator)
    }

    /// Edits the member with the given data. See [`Context::edit_member`] for
    /// more information.
    ///
    /// See [`EditMember`] for the permission(s) required for separate builder
    /// methods, as well as usage of this.
    ///
    /// [`Context::edit_member`]: ../client/struct.Context.html#method.edit_member
    /// [`EditMember`]: ../builder/struct.EditMember.html
    #[cfg(feature="cache")]
    pub fn edit<F: FnOnce(EditMember) -> EditMember>(&self, f: F) -> Result<()> {
        let guild_id = self.find_guild()?;
        let map = f(EditMember::default()).0;

        rest::edit_member(guild_id.0, self.user.read().unwrap().id.0, &map)
    }

    /// Finds the Id of the [`Guild`] that the member is in.
    ///
    /// # Errors
    ///
    /// Returns a [`ClientError::GuildNotFound`] if the guild could not be
    /// found.
    ///
    /// [`ClientError::GuildNotFound`]: ../client/enum.ClientError.html#variant.GuildNotFound
    /// [`Guild`]: struct.Guild.html
    #[cfg(feature="cache")]
    pub fn find_guild(&self) -> Result<GuildId> {
        for guild in CACHE.read().unwrap().guilds.values() {
            let guild = guild.read().unwrap();

            let predicate = guild.members
                .values()
                .any(|m| m.joined_at == self.joined_at && m.user.read().unwrap().id == self.user.read().unwrap().id);

            if predicate {
                return Ok(guild.id);
            }
        }

        Err(Error::Client(ClientError::GuildNotFound))
    }

    /// Removes a [`Role`] from the member, editing its roles in-place if the
    /// request was successful.
    ///
    /// **Note**: Requires the [Manage Roles] permission.
    ///
    /// [`Role`]: struct.Role.html
    /// [Manage Roles]: permissions/constant.MANAGE_ROLES.html
    #[cfg(feature="cache")]
    pub fn remove_role<R: Into<RoleId>>(&mut self, role_id: R) -> Result<()> {
        let role_id = role_id.into();

        if !self.roles.contains(&role_id) {
            return Ok(());
        }

        let guild_id = self.find_guild()?;

        match rest::remove_member_role(guild_id.0, self.user.read().unwrap().id.0, role_id.0) {
            Ok(()) => {
                self.roles.retain(|r| r.0 != role_id.0);

                Ok(())
            },
            Err(why) => Err(why),
        }
    }

    /// Removes one or multiple [`Role`]s from the member.
    ///
    /// **Note**: Requires the [Manage Roles] permission.
    ///
    /// [`Role`]: struct.Role.html
    /// [Manage Roles]: permissions/constant.MANAGE_ROLES.html
    #[cfg(feature="cache")]
    pub fn remove_roles(&mut self, role_ids: &[RoleId]) -> Result<()> {
        let guild_id = self.find_guild()?;
        self.roles.retain(|r| !role_ids.contains(r));

        let map = EditMember::default().roles(&self.roles).0;

        match rest::edit_member(guild_id.0, self.user.read().unwrap().id.0, &map) {
            Ok(()) => Ok(()),
            Err(why) => {
                self.roles.extend_from_slice(role_ids);

                Err(why)
            },
        }
    }

    /// Retrieves the full role data for the user's roles.
    ///
    /// This is shorthand for manually searching through the CACHE.
    ///
    /// If role data can not be found for the member, then `None` is returned.
    #[cfg(feature="cache")]
    pub fn roles(&self) -> Option<Vec<Role>> {
        CACHE.read().unwrap()
            .guilds
            .values()
            .find(|guild| guild
                .read()
                .unwrap()
                .members
                .values()
                .any(|m| m.user.read().unwrap().id == self.user.read().unwrap().id && m.joined_at == *self.joined_at))
            .map(|guild| guild
                .read()
                .unwrap()
                .roles
                .values()
                .filter(|role| self.roles.contains(&role.id))
                .cloned()
                .collect())
    }

    /// Unbans the [`User`] from the guild.
    ///
    /// **Note**: Requires the [Ban Members] permission.
    ///
    /// # Errors
    ///
    /// If the `cache` is enabled, returns a [`ClientError::InvalidPermissions`]
    /// if the current user does not have permission to perform bans.
    ///
    /// [`ClientError::InvalidPermissions`]: ../client/enum.ClientError.html#variant.InvalidPermissions
    /// [`User`]: struct.User.html
    /// [Ban Members]: permissions/constant.BAN_MEMBERS.html
    #[cfg(feature="cache")]
    pub fn unban(&self) -> Result<()> {
        rest::remove_ban(self.find_guild()?.0, self.user.read().unwrap().id.0)
    }
}

impl Display for Member {
    /// Mentions the user so that they receive a notification.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // assumes a `member` has already been bound
    /// println!("{} is a member!", member);
    /// ```
    ///
    // This is in the format of `<@USER_ID>`.
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(&self.user.read().unwrap().mention(), f)
    }
}
