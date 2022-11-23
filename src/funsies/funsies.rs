use rand::Rng;

pub fn squid() -> &'static str {
    "＜コ:彡"
}

pub fn rules() -> &'static str {
    r#"Say it with me now:
    				Watch your back.
    				Shoot straight.
    				Conserve ammo.
    				And never, ever deal with a dragon!
    (ﾉ≧∀≦)ﾉ"#
}

pub fn unyu() -> &'static str {
    "うにゅうー！"
}

pub fn atom() -> &'static str {
    "(●o≧д≦) Up and atom! ☢ 😤 ☢"
}

pub fn yuru() -> &'static str {
    "https://tenor.com/view/yuru-camp-shima-rin-gif-19870064"
}

pub fn dailydose() -> &'static str {
    let links = match rand::thread_rng().gen_range(0..3) {
        0 => vec![
            "https://twitter.com/yakumosgap/status/1474855343176011779",
            "https://twitter.com/kyomoneko_2/status/1485186556210163713",
            "https://twitter.com/bon_feather/status/1567113632814620676",
            "https://skeb.jp/@Hisona_/works/278",
        ],
        _ => vec![
            "https://twitter.com/nsl_mgh/status/1367171515154800640",
            "https://twitter.com/kyomoneko_2/status/1347468091668762626",
            "https://twitter.com/Vtcsku3HJBR2eZw/status/1519303729417097216",
            "https://www.pixiv.net/artworks/92405212",
            "https://www.pixiv.net/artworks/65369107",
            "https://www.pixiv.net/artworks/52084017",
            "https://www.pixiv.net/artworks/1291597",
            "https://www.pixiv.net/artworks/56980398\nhttps://cdn.donmai.us/original/8e/fc/__hakurei_reimu_and_hieda_no_akyuu_touhou_drawn_by_kayako_tdxxxk__8efc14dafa57afd806c95f1ead3ffa8a.jpg",
            "https://www.pixiv.net/artworks/44898062",
            "https://twitter.com/k0mamid0ri/status/815561664477073411",
            "https://twitter.com/smalllightAON/status/1532765221240160261",
            "https://twitter.com/smalllightAON/status/1140648588097331200",
            "https://twitter.com/smalllightAON/status/1310000932634218497",
            "https://twitter.com/Vtcsku3HJBR2eZw/status/1562432126091268097",
            "https://twitter.com/Vtcsku3HJBR2eZw/status/1561344299702165504",
            "https://twitter.com/Vtcsku3HJBR2eZw/status/1566211831143604225",
            "https://avlvla.tumblr.com/post/700762577625645056/when-her-narrative-echoes-with-yours",
            "https://avlvla.tumblr.com/post/691729454195752960/touhou-ship-week-2022-day-5-rarepair",
        ]
    };
    let random_index = rand::thread_rng().gen_range(0..links.len());
    links[random_index]
}
