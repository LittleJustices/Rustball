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
    let links = match rand::thread_rng().gen_range(0..2) {
        0 => vec![
            "https://fxtwitter.com/yakumosgap/status/1474855343176011779",
            "https://fxtwitter.com/kyomoneko_2/status/1485186556210163713",
            "https://fxtwitter.com/bon_feather/status/1567113632814620676",
            "https://fxtwitter.com/bon_feather/status/1620748694147706880",
            "https://fxtwitter.com/bon_feather/status/1626925749629321217",
            "https://fxtwitter.com/bon_feather/status/1672803801978269696",
            "https://skeb.jp/@Hisona_/works/278\nhttps://media.discordapp.net/attachments/245782979710812161/1044659249009135616/951808-1.output.png",
            "https://fxtwitter.com/kyomoneko_2/status/1595693938798567425",
        ],
        _ => vec![
            "https://fxtwitter.com/nsl_mgh/status/1367171515154800640",
            "https://fxtwitter.com/kyomoneko_2/status/1347468091668762626",
            "https://fxtwitter.com/Vtcsku3HJBR2eZw/status/1519303729417097216",
            "https://www.phixiv.net/artworks/92405212",
            "https://www.phixiv.net/artworks/65369107",
            "https://www.phixiv.net/artworks/52084017",
            "https://www.phixiv.net/artworks/1291597",
            "https://www.phixiv.net/artworks/56980398\nhttps://cdn.donmai.us/original/8e/fc/__hakurei_reimu_and_hieda_no_akyuu_touhou_drawn_by_kayako_tdxxxk__8efc14dafa57afd806c95f1ead3ffa8a.jpg",
            "https://www.phixiv.net/artworks/44898062",
            "https://fxtwitter.com/k0mamid0ri/status/815561664477073411",
            "https://fxtwitter.com/smalllightAON/status/1532765221240160261",
            "https://fxtwitter.com/smalllightAON/status/1140648588097331200",
            "https://fxtwitter.com/smalllightAON/status/1310000932634218497",
            "https://fxtwitter.com/smalllightAON/status/1605932609996873728",
            "https://fxtwitter.com/Vtcsku3HJBR2eZw/status/1562432126091268097",
            "https://fxtwitter.com/Vtcsku3HJBR2eZw/status/1561344299702165504",
            "https://fxtwitter.com/Vtcsku3HJBR2eZw/status/1566211831143604225",
            "https://que-de-metal.tumblr.com/post/700762577625645056/when-her-narrative-echoes-with-yours",
            "https://que-de-metal.tumblr.com/post/691729454195752960/touhou-ship-week-2022-day-5-rarepair",
            "https://www.tumblr.com/amemenojaku/710993007892168704/for-the-rarepair-suggestions-maybe-akyuurei-im",
            "https://fxtwitter.com/PileArcato/status/1640618804559372293",
            "https://fxtwitter.com/smalllightAON/status/1625319312700350468",
            "https://fxtwitter.com/PileArcato/status/1641288827368099841",
            "https://fxtwitter.com/PileArcato/status/1641120552373960707",
            "https://fxtwitter.com/PileArcato/status/1658374773674475520",
            "https://fxtwitter.com/Kuzukago_123/status/1646491966434349056",
            "https://fxtwitter.com/31mriri0830/status/1652673110141976576",
            "https://fxtwitter.com/smalllightAON/status/1676784121618497538",
            "https://que-de-metal.tumblr.com/post/724974783611207680/you-wrote-what-touhou-ship-week-2023-day-1",
            "https://que-de-metal.tumblr.com/post/725119736406589440/touhou-ship-week-2023-oops-all-akyuurei-day-2",
            "https://que-de-metal.tumblr.com/post/725205855230902272/touhou-ship-week-2023-oops-all-akyuurei-day-3",
            "https://que-de-metal.tumblr.com/post/725287210600087553/touhou-ship-week-2023-oops-all-akyuurei-day-4",
            "https://que-de-metal.tumblr.com/post/725372344029315072/touhou-ship-week-2023-oops-all-akyuurei-day-5",
            "https://que-de-metal.tumblr.com/post/725438351395700736/touhou-ship-week-2023-oops-all-akyuurei-day-6",
        ]
    };
    let random_index = rand::thread_rng().gen_range(0..links.len());
    links[random_index]
}
