use crate::log_channel::{log_voice_chat_joined, log_voice_chat_left, log_voice_chat_moved};
use serenity::model::id::ChannelId;
use serenity::model::user::User;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;

use crate::utils::logging::log_error;

use super::errors::VoiceStateUpdateError;

pub enum VoiceAction {
    UserJoinedChannel,
    UserLeftChannel,
    UserMovedChannel,
    Unknown,
}

pub async fn handle(ctx: Context, old: Option<VoiceState>, new: VoiceState) {
    let action = VoiceAction::new(&old, &new);
    let data = match action {
        VoiceAction::UserJoinedChannel => VoiceAction::joined_channel(new),
        VoiceAction::UserLeftChannel => VoiceAction::left_channel(old, new),
        VoiceAction::UserMovedChannel => VoiceAction::moved_channel(old, new),
        VoiceAction::Unknown => Err(VoiceStateUpdateError::Other(
            "Unknown action voice action captured.".to_string(),
        )),
    };

    match data {
        Ok(data) => send_log(ctx, action, data).await,
        Err(e) => log_error(&e),
    }
}

type LogData = (User, Vec<ChannelId>);

pub async fn send_log(ctx: Context, log_type: VoiceAction, data: LogData) {
    match log_type {
        VoiceAction::UserJoinedChannel => {
            match log_voice_chat_joined(data.0, data.1[0], &ctx).await {
                Ok(_) => {}
                Err(e) => log_error(&e),
            }
        }
        VoiceAction::UserLeftChannel => match log_voice_chat_left(data.0, data.1[0], &ctx).await {
            Ok(_) => {}
            Err(e) => log_error(&e),
        },
        VoiceAction::UserMovedChannel => {
            match log_voice_chat_moved(data.0, data.1[1], data.1[0], &ctx).await {
                Ok(_) => {}
                Err(e) => log_error(&e),
            }
        }
        _ => println!("Voice update happened..."),
    }
}

impl VoiceAction {
    fn new(old: &Option<VoiceState>, new: &VoiceState) -> Self {
        let old_has_channel_id = old.is_some();
        let new_has_channel_id = new.channel_id.is_some();

        if old_has_channel_id && new_has_channel_id {
            Self::UserMovedChannel
        } else if !old_has_channel_id && new_has_channel_id {
            Self::UserJoinedChannel
        } else if old_has_channel_id && !new_has_channel_id {
            Self::UserLeftChannel
        } else {
            Self::Unknown
        }
    }

    fn joined_channel(new: VoiceState) -> Result<(User, Vec<ChannelId>), VoiceStateUpdateError> {
        let user = match new.member {
            Some(x) => x.user,
            None => {
                return Err(VoiceStateUpdateError::DataMissing(
                    "Member from new VoiceState missing".to_string(),
                ))
            }
        };

        let channel = match new.channel_id {
            Some(x) => x,
            None => {
                return Err(VoiceStateUpdateError::DataMissing(
                    "ChannelId from new VoiceState missing".to_string(),
                ))
            }
        };

        Ok((user, vec![channel]))
    }

    fn left_channel(
        old: Option<VoiceState>,
        new: VoiceState,
    ) -> Result<(User, Vec<ChannelId>), VoiceStateUpdateError> {
        let user = match new.member {
            Some(x) => x.user,
            None => {
                return Err(VoiceStateUpdateError::DataMissing(
                    "Member from new VoiceState missing".to_string(),
                ))
            }
        };

        let channel = match old {
            Some(x) => match x.channel_id {
                Some(x) => x,
                None => {
                    return Err(VoiceStateUpdateError::DataMissing(
                        "ChannelId from new VoiceState missing".to_string(),
                    ))
                }
            },
            None => {
                return Err(VoiceStateUpdateError::DataMissing(
                    "Old VoiceState missing".to_string(),
                ))
            }
        };

        Ok((user, vec![channel]))
    }

    fn moved_channel(
        old: Option<VoiceState>,
        new: VoiceState,
    ) -> Result<(User, Vec<ChannelId>), VoiceStateUpdateError> {
        let user = match new.member {
            Some(x) => x.user,
            None => {
                return Err(VoiceStateUpdateError::DataMissing(
                    "Member from new VoiceState missing".to_string(),
                ))
            }
        };

        let new_channel = match new.channel_id {
            Some(x) => x,
            None => {
                return Err(VoiceStateUpdateError::DataMissing(
                    "ChannelId from new VoiceState missing".to_string(),
                ))
            }
        };

        let old_channel = match old {
            Some(x) => match x.channel_id {
                Some(x) => x,
                None => {
                    return Err(VoiceStateUpdateError::DataMissing(
                        "ChannelId from new VoiceState missing".to_string(),
                    ))
                }
            },
            None => {
                return Err(VoiceStateUpdateError::DataMissing(
                    "Old VoiceState missing".to_string(),
                ))
            }
        };

        Ok((user, vec![new_channel, old_channel]))
    }
}
