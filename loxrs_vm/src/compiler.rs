use std::vec;

use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

use crate::{
    error::{Label, Result},
    lexer::Lexer,
};

pub fn compile(source: &str) -> Result<()> {
    let lexer = Lexer::new(source);
    let mut labels: Vec<Label> = vec![];
    for el in lexer.into_iter() {
        match el {
            Ok(token) => println!("{:?}", token),
            Err(err) => {
                println!("{:?}", err);
                labels.push((err.0.into(), err.1).into());
            }
        }
    }

    let file = SimpleFile::new("source", source);
    let diagnostic: Diagnostic<()> = Diagnostic::error()
        .with_message("Lexer error")
        .with_labels(labels.iter().map(|el| el.0.clone()).collect());

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    if !labels.is_empty() {
        term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
    }

    Ok(())
}
