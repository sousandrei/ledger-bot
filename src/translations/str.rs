

mod en;
mod br;

enum Language {
    EN,
    BR,
}

// Todo, change this value based on settings
const lan: String = Language::BR;

fn intro() -> String {
    match lan {
        Language::EN => en::intro(),
        Language::BR => br::intro(),
    }
}