use random_ramble::refactor::RandomRamble;

use notify_rust::Notification;

static SOUND: &str = "message-new-instant";

fn main() {
    let timeout = 20;

    let summary = RandomRamble::new()
        .with_template("{{ intro | rr }}\n{{ desc | rr }}")
        .with_rambles(
            "intro",
            vec!["Stop right there, criminal scum!", "<snarky remark here>"],
        )
        .with_rambles(
            "desc",
            vec![
                "Get up! Lazy <noun>",
                "You'll die if you don't move!",
                "You've been pretending to work for like 20 minutes! Take it easy.",
                "Hands off the keyboard.",
                "Hands up!! Now, wave them around, like you just don't care",
                "Something method Pomodoro, something…",
                "You remind me of the humans in Wall-E… GET UP!",
            ],
        )
        .build()
        .unwrap()
        .to_string();

    let btn_txt = RandomRamble::new()
        .with_template("{{ btn | rr }}")
        .with_rambles(
            "btn",
            vec![
                "Feck off",
                "Leave me alone!",
                "STFU",
                "Honey, I'm not in the mood.",
                "SNOOZE!!!!",
                "OMG, I don't care",
                "Shh, only dreams now…",
                "Help, I'm being repressed!",
            ],
        )
        .build()
        .unwrap()
        .to_string();

    #[cfg(all(unix, not(target_os = "macos")))]
    Notification::new()
        .icon("chronometer")
        .sound_name(SOUND)
        .summary(&summary)
        .appname("Get up, NERD!")
        .action("snooze", &btn_txt)
        .timeout(timeout * 1000)
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "default" => println!("default"),
            "snooze" => println!("<put code here>"),
            // FIXME: here "__closed" is a hardcoded keyword, it will be deprecated!!
            "__closed" => println!("the notification was closed"),
            _ => (),
        });

    Notification::new()
        .appname("Back to work, NERD!")
        .icon("display-symbolic")
        .summary("Alright, back to work.")
        .timeout(3 * 1000)
        .show()
        .unwrap();
}
