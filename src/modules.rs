pub trait Module {
    fn fmt<'a>(opts: &Option<Vec<&'a str>>) -> String;
}
