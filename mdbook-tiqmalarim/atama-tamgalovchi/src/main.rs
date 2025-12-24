use atama_tamgalovchi::AtamaTamgalovchi;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::io;

fn main() {
    let mut args = std::env::args().skip(1);
    if let Some("supports") = args.next().as_deref() {
        return;
    }
    if let Err(e) = oldishlovni_yurit() {
        eprintln!("Atama Tamg'alovchi Xatosi: {}", e);
        std::process::exit(1);
    }
}

pub fn oldishlovni_yurit() -> mdbook::errors::Result<()> {
    let mut kiritma = io::stdin();

    // vaziyat = ctx
    let (vaziyat, kitob) = CmdPreprocessor::parse_input(&mut kiritma)?;
    let ishlov_berilmish_kitob = AtamaTamgalovchi.run(&vaziyat, kitob)?;

    let chiqarma = io::stdout();
    let mut chiqarma_qulfi = chiqarma.lock();
    serde_json::to_writer(&mut chiqarma_qulfi, &ishlov_berilmish_kitob)?;
    Ok(())
}
