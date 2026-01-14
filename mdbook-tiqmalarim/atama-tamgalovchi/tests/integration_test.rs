use atama_tamgalovchi::AtamaTamgalovchi;
use mdbook::book::{Book, BookItem, Chapter};
use std::fs;
use std::path::PathBuf;

#[test]
fn sina_tamgalovchi_jumlani_almashtiradimi() {
    let muvaqqat_jild = tempfile::tempdir().unwrap();
    let terms_yolagi = muvaqqat_jild.path().join("terms");
    fs::create_dir(&terms_yolagi).unwrap();

    // Tamgʻalovchi oʻz ma'lumotlar omborida saqlaydigan atama fayli
    let atama_matni = r#"# API
**Inglizça:** Application Programming Interface
**Rusça:** API
**Qisqartma:** API"#;
    fs::write(terms_yolagi.join("api.md"), atama_matni).unwrap();

    let mut kitob = Book::new();

    // tamgʻalamoqchi boʻlayotgan matnimiz
    let matn = r#"# Kirish
**Inglizça:** Oʻrin Egallovchi

Men har kuni API ishlataman.
```rust,ignore
const API = 1;
```
`API` bu dahshat.

## Bogʻlamalar
"#;

    // DIQQAT: Yoʻlak "terms/" jumlasi bilan boshlanishi shart
    kitob.sections.push(BookItem::Chapter(Chapter::new(
        "Kirish",
        matn.to_string(),
        PathBuf::from("terms/intro.md"),
        vec![],
    )));

    let tamgalovchi = AtamaTamgalovchi::new();
    let natija = tamgalovchi
        .run_internal(kitob, Some(&terms_yolagi))
        .unwrap();

    let kirish = match &natija.sections[0] {
        BookItem::Chapter(c) => c,
        _ => panic!("Bu bob emas"),
    };

    // 1. Oddiy matn tamgʻalanganligini tekshir
    assert!(
        kirish
            .content
            .contains(r#"<abbr data-en-ru="Application Programming Interface / API" aria-label="Application Programming Interface / API">API</abbr>"#),
        "'API' oddiy jumlasi tamgʻalanmagan ekan. Matn quyidagicha edi:\n{}",
        kirish.content
    );

    // 2. Bitik boʻlagiga teginilmaganini tekshir (ichida tamgʻa yoʻq)
    assert!(kirish.content.contains("const API = 1;"));

    // 3. Satrichi bitigi teginilmaganini tekshir
    assert!(kirish.content.contains("`API` bu dahshat."));
}

#[test]
fn sina_sifat_shakli_tamgalandimi() {
    let muvaqqat_jild = tempfile::tempdir().unwrap();
    let terms_yolagi = muvaqqat_jild.path().join("terms");
    fs::create_dir(&terms_yolagi).unwrap();

    let atama_matni = r#"# Atamalilik
**Inglizça:** Term
**Rusça:** Термин
**Sifat şakli:** atamali"#;
    fs::write(terms_yolagi.join("atama.md"), atama_matni).unwrap();

    let mut kitob = Book::new();

    let matn = r#"# Sifatlarni Sinash
**Inglizça:** Test

Ushbu gapda atamali birikma bor.

## Summary
"#;

    // DIQQAT: Yoʻlak "terms/" jumlasi bilan boshlanishi shart
    kitob.sections.push(BookItem::Chapter(Chapter::new(
        "Sifat Sinovi",
        matn.to_string(),
        PathBuf::from("terms/test.md"),
        vec![],
    )));

    let tamgalovchi = AtamaTamgalovchi::new();
    let natija = tamgalovchi
        .run_internal(kitob, Some(&terms_yolagi))
        .unwrap();

    let bob = match &natija.sections[0] {
        BookItem::Chapter(b) => b,
        _ => panic!("bu bob emas"),
    };

    // Tamgʻalovchi "sifat shakli"dagi atamani ham tamgʻalaganligini tekshir
    assert!(
        bob.content.contains(
            r#"<abbr data-en-ru="Term / Термин" aria-label="Term / Термин">atamali</abbr>"#
        ),
        "'atamali' Sifat shakli  tamgʻalanmagan ekan. Matn quyidagicha edi:\n{}",
        bob.content
    );
}
