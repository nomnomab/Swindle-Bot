use serenity::model::channel::Message;

static KARMA_CHECKS: [&str; 8] = [
    "thx",
    "thanks",
    "gracias",
    "Спасибо",
    "감사",
    "धन्यवाद",
    "ありがとう",
    "Merci"];

pub fn check_for_karma(msg: &Message) -> bool {
    let content = &msg.content.to_lowercase();
    let mut valid = false;
    for check in &KARMA_CHECKS {
        valid = content.contains(check);

        if valid {
            return true;
        }
    }

    false
}