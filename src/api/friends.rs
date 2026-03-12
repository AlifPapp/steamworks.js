use napi_derive::napi;

#[napi]
pub mod friends {
    use napi::bindgen_prelude::BigInt;
    use steamworks::SteamId;

    /// Returns the display name for a given Steam user.
    ///
    /// If the name is not cached locally, this returns an empty string or
    /// "[unknown]".  Call `request_user_information` first and wait for the
    /// `PersonaStateChange` callback before retrying.
    ///
    /// {@link https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendPersonaName}
    #[napi]
    pub fn get_persona_name(steam_id64: BigInt) -> String {
        let client = crate::client::get_client();
        let steam_id = SteamId::from_raw(steam_id64.get_u64().1);
        let friend = client.friends().get_friend(steam_id);
        friend.name()
    }

    /// Requests persona data (display name, avatar) from Steam servers for a
    /// user that is not in the local cache.
    ///
    /// Returns `true` if information needs to be fetched from the server
    /// (a `PersonaStateChange` callback will fire when ready).
    /// Returns `false` if the information is already available locally.
    ///
    /// {@link https://partner.steamgames.com/doc/api/ISteamFriends#RequestUserInformation}
    #[napi]
    pub fn request_user_information(steam_id64: BigInt, name_only: bool) -> bool {
        let client = crate::client::get_client();
        let steam_id = SteamId::from_raw(steam_id64.get_u64().1);
        client.friends().request_user_information(steam_id, name_only)
    }
}
