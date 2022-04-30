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
    let links = vec![
        "https://fxtwitter.com/nsl_mgh/status/1367171515154800640",
        "https://twitter.com/kyomoneko_2/status/1347468091668762626",
        "https://twitter.com/yakumosgap/status/1474855343176011779",
        "https://twitter.com/kyomoneko_2/status/1485186556210163713",
        "https://twitter.com/Vtcsku3HJBR2eZw/status/1519303729417097216",
    ];
    let random_index = rand::thread_rng().gen_range(0..links.len());
    links[random_index]
}