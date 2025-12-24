use atama_orinlovchi::AtamaOrinlovchi;
use mdbook::errors::Result;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::io;

fn main() {
    let mut args = std::env::args().skip(1);
    if let Some("supports") = args.next().as_deref() {
        return;
    }
    if let Err(e) = oldishlovni_yurit() {
        eprintln!("Orinlovchi xatosi: {}", e);
        std::process::exit(1);
    }
}

pub fn oldishlovni_yurit() -> Result<()> {
    let mut kiritma = io::stdin();

    // vaziyat = ctx
    let (vaziyat, kitob) = CmdPreprocessor::parse_input(&mut kiritma)?;
    let ishlov_berilmish_kitob = AtamaOrinlovchi.run(&vaziyat, kitob)?;

    let chiqarma = io::stdout();
    let mut chiqarma_qulfi = chiqarma.lock();
    serde_json::to_writer(&mut chiqarma_qulfi, &ishlov_berilmish_kitob)?;
    Ok(())
}
