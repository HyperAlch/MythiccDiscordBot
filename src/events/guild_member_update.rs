use serenity::client::Context;
use serenity::model::prelude::Member;

use crate::log_channel::log_roles_updated;
use crate::utils::logging::log_error;

use super::errors::GuildMemberUpdateError;

pub async fn handle(old_if_available: Option<Member>, new: Member, ctx: &Context) {
    let old_roles_state = match old_if_available {
        Some(state) => state.roles,
        None => {
            log_error(&GuildMemberUpdateError::DataMissing(
                "old_roles_state".to_string(),
            ));
            return;
        }
    };

    let new_roles_state = new.roles;

    let mut new_roles = Vec::new();
    let mut old_roles = Vec::new();

    for x in new_roles_state.iter() {
        if !old_roles_state.contains(x) {
            new_roles.push(x);
        }
    }

    for x in old_roles_state.iter() {
        if !new_roles_state.contains(x) {
            old_roles.push(x);
        }
    }

    let new_roles: Vec<String> = new_roles
        .iter()
        .map(|role| {
            let mut role = role.to_string();
            role.insert_str(0, "<@&");
            role.insert(role.len(), '>');
            role
        })
        .collect();

    let old_roles: Vec<String> = old_roles
        .iter()
        .map(|role| {
            let mut role = role.to_string();
            role.insert_str(0, "<@&");
            role.insert(role.len(), '>');
            role
        })
        .collect();

    if !old_roles.is_empty() || !new_roles.is_empty() {
        match log_roles_updated(new.user, new_roles, old_roles, ctx).await {
            Ok(_) => (),
            Err(error) => log_error(&error),
        };
    }
}
