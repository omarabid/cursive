use cursive::views::{Dialog, Text};
use cursive::Cursive;

fn main() {
    let mut siv = Cursive::default();

    #[cfg(feature = "toml")]
    {
        // You can load a theme from a file at runtime for fast development.
        siv.load_theme_file("assets/style.toml").unwrap();

        // Or you can directly load it from a string for easy deployment.
        siv.load_toml(include_str!("../assets/style.toml")).unwrap();

        siv.add_layer(
            Dialog::around(Text::new(
                "This application uses a \
                 custom theme!",
            ))
            .title("Themed dialog")
            .button("Quit", |s| s.quit()),
        );
    }

    #[cfg(not(feature = "toml"))]
    {
        siv.add_layer(
            Dialog::around(Text::new(
                "Run this example again with the `toml` feature!\n\n\
                 cargo run --example theme --features toml",
            ))
            .title("Themed dialog - missing `toml` feature")
            .button("Quit", |s| s.quit()),
        );
    }

    siv.run();
}
