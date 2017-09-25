#[derive(Default)]
struct TextBoxSettings {
    title: String,
    text: String,
    speed: String,
    bg: String,
    border: String,
    sound: String,
    color: String,
    anim: String,
    theme: String,
    target: String,
    direction: String,
    width: u16,
    height: u16,
    wrap: bool,
    resize: bool,
}

impl TextBoxSettings {
    fn new() -> TextBoxSettings {
        TextBoxSettings {
            title: "Egg".to_string(),
            ..Default::default()
        }
    }
}



//#aaff03Some Text#fa30b3 Text in another color
//[color = red]Text[/color] [b]Bold text[/b] [i]Slanted[/i] [u]Undertaled[/u]
