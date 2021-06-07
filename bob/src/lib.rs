pub fn reply(message: &str) -> &str {
    let response = match (
        message.ends_with("."),
        message.ends_with("?"),
        message.ends_with(" "),
        message.ends_with("!"),
    ) {
        (true, _, _, _) => "Whatever.",
        (_, _, true, _) => "Sure.",
        (_, _, _, true) => "Whoa, chill out!",
        (_, _, _, _) => "",
    };
    response
}
