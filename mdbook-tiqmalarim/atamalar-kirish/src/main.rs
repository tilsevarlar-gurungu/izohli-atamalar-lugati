use atamalar_kirish::KirishOldIshlovchisi;
use mdbook::preprocess::CmdPreprocessor;
use std::io;

fn main() {
    let oldishlovchi = KirishOldIshlovchisi::new();

    if std::env::args().nth(1).as_deref() == Some("supports") {
        std::process::exit(0);
    }

    if let Err(e) = oldishlovni_yurit(&oldishlovchi) {
        eprintln!("Atamalar kirishida xato: {}", e);
        std::process::exit(1);
    }
}

fn oldishlovni_yurit(oldishlovchi: &KirishOldIshlovchisi) -> anyhow::Result<()> {
    // vaziyat = ctx
    let (vaziyat, kitob) = CmdPreprocessor::parse_input(io::stdin())?;

    // 1. vaziyat.root (mutlaq) ni kitobning src va
    // atamalar yo'lagi bilan qo'shing
    let terms_jildi = vaziyat.root.join(&vaziyat.config.book.src).join("terms");

    // 2. Aniqroq xato matni uchun, jildning mavjudligini tekshiring
    if !terms_jildi.exists() {
        return Err(anyhow::anyhow!(
            "{} da Terms jildi topilmadi. book.toml'ning [book] src moslamasini tekshiring",
            terms_jildi.display()
        ));
    }

    let ishlov_berilmish_kitob = oldishlovchi.run_internal(kitob, &terms_jildi)?;
    serde_json::to_writer(io::stdout(), &ishlov_berilmish_kitob)?;

    Ok(())
}
