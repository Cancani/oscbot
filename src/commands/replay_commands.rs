use poise::serenity_prelude as serenity;
use rosu_v2::prelude::BeatmapExtended;
use crate::discord_helper::MessageState;
use crate::embeds::single_text_response;
use crate::{Context, Error, embeds};
use rosu_v2::{Osu, prelude as rosu};

use crate::osu;
use crate::generate::thumbnail;
use crate::discord_helper::user_has_replay_role;

async fn has_replay_role(ctx: Context<'_>) -> Result<bool, Error> {
    if !user_has_replay_role(ctx, ctx.author()).await.unwrap() {
        ctx.say("No permission L").await?;
        return Ok(false);
    }
    Ok(true)
}

#[poise::command(slash_command, rename = "replay", subcommands("generate"), check = "has_replay_role")]
pub async fn bundle(_ctx: Context<'_>, _arg: String) -> Result<(), Error> { Ok(()) }

#[poise::command(slash_command, subcommands("thumbnail"), check = "has_replay_role")]
pub async fn generate(_ctx: Context<'_>, _arg: String) -> Result<(), Error> { Ok(()) }

/// Either select score id or score file
#[poise::command(slash_command)]
pub async fn thumbnail(
    ctx: Context<'_>,
    #[description = "score id"] scoreid: Option<u64>,
    #[description = "score file"] scorefile: Option<serenity::Attachment>,
    #[description = "subtitle inside the thumbnail"] subtitle: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let mut image: Vec<u8>;

    if scoreid.is_some() {
        let unwrapped_score_id = scoreid.unwrap();
        let score = match osu::get_osu_instance().score(unwrapped_score_id).await {
            Ok(score) => score,
            Err(_) => {
                ctx.send(poise::CreateReply::default().embed(serenity::CreateEmbed::default().description(format!("Score with id {} does not exist", unwrapped_score_id)))).await?;
                return Ok(());
            }
        };
        let map = osu::get_osu_instance().beatmap().map_id(score.map_id).await.expect("Beatmap exists");
        image = thumbnail::generate_thumbnail_from_score(score, map, &subtitle.unwrap_or("".to_string())).await;
    }
    else if scorefile.is_some() {
        let bytes = scorefile.unwrap().download().await?;
        let replay = match osu_db::Replay::from_bytes(&bytes) {
            Ok(replay) => replay,
            Err(_) => {
                single_text_response(&ctx, "Replay could not be parsed", MessageState::ERROR).await;
                return Ok(());
            },
        };
        let default_checksum = "".to_string();
        let checksum = replay.beatmap_hash.as_ref().unwrap_or(&default_checksum);
        let map: BeatmapExtended = match osu::get_osu_instance().beatmap().checksum(checksum).await {
            Ok(map) => map,
            Err(_) => {
                single_text_response(&ctx, "Cannot find map related to the replay", MessageState::WARN).await;
                return Ok(());
            },
        };
        image = thumbnail::generate_thumbnail_from_replay_file(&replay, map, &subtitle.unwrap_or("".to_string())).await;
    }
    else {
        embeds::single_text_response(&ctx, "Please define scoreid or scorefile", MessageState::WARN).await;
        return Ok(());
    }

    ctx.send(poise::CreateReply::default().attachment(serenity::CreateAttachment::bytes(image, "thumbnail.png"))).await?;
    Ok(())
}