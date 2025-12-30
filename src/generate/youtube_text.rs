use rosu_v2::prelude as rosu;

use crate::osu::formatter::mods_string;

pub fn generate_title(score: &rosu::Score, map: &rosu::BeatmapExtended) -> String {
    let mapset = map.mapset.as_ref().expect("missing mapset");
    let user = score.user.as_ref().expect("Fuck you");

    format!("{} | {} - {} [{}] {:.2}⭐ +{}", user.username, mapset.artist, mapset.title, map.version, score.map.as_ref().expect("map to exist").stars, mods_string(&score.mods))
}

pub fn generate_description(score: &rosu::Score) -> String {

    let fmt = time::format_description::parse("[day].[month].[year] at [hour]:[minute]").unwrap();
    format!("
This score was set on {}.

Player: https://osu.ppy.sh/users/{}
Beatmap: https://osu.ppy.sh/beatmaps/{}
Score: https://osu.ppy.sh/scores/{}

Join the osu swiss community in discord: https://discord.com/invite/SHz8QtD", score.ended_at.format(&fmt).unwrap(), score.user_id, score.map_id, score.id)
}