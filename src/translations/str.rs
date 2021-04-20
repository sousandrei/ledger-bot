

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

fn help() -> String {
    match lan {
        Language::EN => en::help(),
        Language::BR => br::help(),
    }
}

fn add() -> String {
    match lan {
        Language::EN => en::add(),
        Language::BR => br::add(),
    }
}

fn list() -> String {
    match lan {
        Language::EN => en::list(),
        Language::BR => br::list(),
    }
}

fn del() -> String {
    match lan {
        Language::EN => en::del(),
        Language::BR => br::del(),
    }
}

fn lend() -> String {
    match lan {
        Language::EN => en::lend(),
        Language::BR => br::lend(),
    }
}