pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb: String = String::from("");
    let mut pointer = 0;
    for i in list {
        if list.len() > 1 && i != &list[list.len() - 1] {
            proverb = format!(
                "{0}\nFor want of a {1} the {2} was lost.",
                proverb,
                &list[pointer],
                &list[pointer + 1],
            );
            pointer += 1;
        } else {
            proverb = format!("{0}\nAnd all for the want of a {1}.\n", proverb, &list[0]);
        }
    }
    format!("{}", proverb.trim())
}
