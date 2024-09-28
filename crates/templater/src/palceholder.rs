pub struct Placeholder<'a> {
    pub name: &'a str,
    pub prefix: Option<&'a str>,
    pub suffix: Option<&'a str>,
    pub delimiter: Option<&'a str>,
}