pub type Command<'a> = Box<FnMut() + 'a>;
