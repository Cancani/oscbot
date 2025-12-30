use rosu_v2::prelude::{self as rosu, GameModIntermode, GameMods, NoFailOsu};

use rosu_v2::prelude::GameMod;

fn convert_osu_db_mod_to_string(m: osu_db::Mod) -> &'static str {
    match m {
        // osu!
        osu_db::Mod::NoFail => "NF",
        osu_db::Mod::Easy => "EZ",
        osu_db::Mod::TouchDevice => "TD",
        osu_db::Mod::Hidden => "HD",
        osu_db::Mod::HardRock => "HR",
        osu_db::Mod::SuddenDeath => "SD",
        osu_db::Mod::DoubleTime => "DT",
        osu_db::Mod::Relax => "RX",
        osu_db::Mod::HalfTime => "HT",
        osu_db::Mod::Nightcore => "NC",
        osu_db::Mod::Flashlight => "FL",
        osu_db::Mod::Autoplay => "AT",
        osu_db::Mod::SpunOut => "SO",
        osu_db::Mod::Autopilot => "AP",
        osu_db::Mod::Perfect => "PF",
        osu_db::Mod::Random => "RD",
        osu_db::Mod::LastMod => "CN",
        osu_db::Mod::TargetPractice => "TP",
        // mania
        osu_db::Mod::Key1 => "1K",
        osu_db::Mod::Key2 => "2K",
        osu_db::Mod::Key3 => "3K",
        osu_db::Mod::Key4 => "4K",
        osu_db::Mod::Key5 => "5K",
        osu_db::Mod::Key6 => "6K",
        osu_db::Mod::Key7 => "7K",
        osu_db::Mod::Key8 => "8K",
        osu_db::Mod::Key9 => "9K",
        osu_db::Mod::Coop => "CO",
        osu_db::Mod::FadeIn => "FI",
    }
}

pub fn map_title(map: &rosu::BeatmapExtended) -> String {
    let mapset = map.mapset.as_deref().expect("missing mapset");
    format!("{} - {} [{}]", mapset.artist, mapset.title, map.version)
}

pub fn osu_hits(score_statistics: &rosu::ScoreStatistics) -> String {
    format!("{}/{}/{}/{}", score_statistics.great, score_statistics.ok, score_statistics.meh, score_statistics.miss)
}

pub fn score_url(score_id: &u64) -> String {
    format!("https://osu.ppy.sh/scores/{}", score_id.to_string())
}

pub fn mods_string(mods: &rosu::GameMods) -> String {
    mods.iter().map(|map: &rosu::GameMod| map.acronym().to_string()).collect::<Vec<_>>().join("")
}

pub fn convert_osu_db_to_mod_array(mods: osu_db::ModSet) -> Vec<String> {
    let mut x = mods.bits();
    let mut mod_array: Vec<String> = Vec::new();
    while x != 0 {
        let bit = x & x.wrapping_neg();
        let intermode = GameModIntermode::try_from_bits(bit).unwrap();
        mod_array.push(intermode.acronym().as_str().to_string());
        x &= x - 1;
    };
    mod_array
}

pub fn calculate_grade_from_accuracy(accuracy: f32, has_miss: bool, hidden: bool) -> rosu::Grade {
    if accuracy == 100.0 {
        return if hidden {rosu::Grade::XH} else {rosu::Grade::X};
    }

    let true_accuracy = if has_miss {accuracy} else {accuracy + 10.0};

    if 70.0 > true_accuracy {
        return rosu::Grade::D;
    }
    else if 80.0 > true_accuracy {
        return rosu::Grade::C
    }

    else if 90.0 > true_accuracy {
        return rosu::Grade::B;
    }

    else if accuracy > 90.0 && !has_miss {
        return if hidden {rosu::Grade::SH} else {rosu::Grade::S};
    }

    return rosu::Grade::A;
}