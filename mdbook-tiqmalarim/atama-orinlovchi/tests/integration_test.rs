use atama_orinlovchi::AtamaOrinlovchi;
use mdbook::book::{Book, BookItem, Chapter};
use std::fs;
use std::path::PathBuf;

#[test]
fn sina_orinlovchi_korsatkich_yaratadi_va_sahifalarga_kirgizadimi() {
    let muvaqqat_jild = tempfile::tempdir().unwrap();
    let terms_yolagi = muvaqqat_jild.path().join("terms");
    fs::create_dir(&terms_yolagi).unwrap();

    // 1. Oʻchmas xotirada kitob SUMMARY'sida yoʻq atama faylini yarat
    let atama_matni = r#"# Algoritm
**Inglizcha:** Algorithm
**Ruscha:** Алгоритм
**Soha:** Informatika"#;
    fs::write(terms_yolagi.join("algoritm.md"), atama_matni).unwrap();

    // 2. Koʻrsatkich uchun oʻrinni ushlab turuvchisi bor kitob yarat
    let mut kitob = Book::new();
    kitob.sections.push(BookItem::Chapter(Chapter::new(
        "Bosh Sahifa",
        "Oʻrin ushlab turuvchi matn".to_string(),
        PathBuf::from("terms.md"),
        vec![],
    )));

    // 3. Oʻchmas xotira manzili yoʻlagi uchun ichki ust yozuvli
    // oʻrinlovchini ishga tushir
    let orinlovchi = AtamaOrinlovchi::new();
    let natija = orinlovchi.run_internal(kitob, Some(&terms_yolagi)).unwrap();

    // --- Tekshiruv ---

    // Koʻrsatkich bobini top (terms.md)
    let korsatkich_bobi = natija
        .sections
        .iter()
        .find(|unsur| {
            if let BookItem::Chapter(bob) = unsur {
                bob.path
                    .as_ref()
                    .map_or(false, |p| p.to_str() == Some("terms.md"))
            } else {
                false
            }
        })
        .unwrap();

    if let BookItem::Chapter(bob) = korsatkich_bobi {
        // Alifbo tartibini tekshir
        assert!(bob.content.contains("## <span id=\"a\"></span>A"));
        // Atama maqolasida Soha va tarjimalar borligini tekshir
        assert!(bob.content.contains("[**Algoritm**](terms/algoritm.md)"));
        assert!(bob.content.contains("<small><i>[Informatika]</i></small>"));
        assert!(bob.content.contains("— Algorithm — Алгоритм"));
        // Yoʻltanim qatorini tekshir
        assert!(bob.content.contains("href=\"#a\""));
    }

    // "Algoritm" sahifa kitobning "sections" qatoriga qoʻshilganini tekshir
    let kirgizilmish = natija.sections.iter().any(|unsur| {
        if let BookItem::Chapter(bob) = unsur {
            bob.name == "Algoritm"
                && bob
                    .path
                    .as_ref()
                    .map_or(false, |p| p.to_str() == Some("terms/algoritm.md"))
        } else {
            false
        }
    });
    assert!(kirgizilmish, "Atama maqolasi kitob boʻlimiga kiritilmabdi");
}

#[test]
fn sina_orinlovchi_qisqartma_va_pogonalarni_yuritadimi() {
    let muvaqqat_jild = tempfile::tempdir().unwrap();
    let terms_yolagi = muvaqqat_jild.path().join("terms");
    fs::create_dir(&terms_yolagi).unwrap();

    let ota_matn = r#"# API
**Inglizcha:** Application Programming Interface
**Ruscha:** API
**Qisqartma:** API"#;
    fs::write(terms_yolagi.join("api.md"), ota_matn).unwrap();

    let bola_matn = r#"# REST API
**Inglizcha:** REST API
**Ruscha:** REST API
**Ota-atama:** API"#;
    fs::write(terms_yolagi.join("rest.md"), bola_matn).unwrap();

    let kitob = Book::new();
    let orinlovchi = AtamaOrinlovchi::new();
    let natija = orinlovchi.run_internal(kitob, Some(&terms_yolagi)).unwrap();

    // Koʻrsatkich odatda ilk boʻlim, chunki u 0 oʻrin raqami bilan qoʻshilgan
    let korsatkich_bobi = natija
        .sections
        .iter()
        .find(|unsur| {
            if let BookItem::Chapter(bob) = unsur {
                bob.name == "Oʻzbekcha - Inglizcha - Ruscha Koʻrsatkich"
            } else {
                false
            }
        })
        .map(|unsur| match unsur {
            BookItem::Chapter(bob) => bob,
            _ => unreachable!(),
        })
        .expect("Koʻrsatkich bobi topilmadi");

    // 1. qisqartmani tekshir
    assert!(
        korsatkich_bobi.content.contains("(API)"),
        "(API) qisqartmasi topilmadi"
    );

    // 2. Pogʻonalilikni tekshir
    // Biz havolarlarning qancha chekinmaga egaligini tekshiramiz
    // Bu bitik checkinma uchun 4 boʻshliq ishlatadi.
    let kutilgan_maqola = "* [REST API](terms/rest.md)";
    assert!(
        korsatkich_bobi.content.contains(kutilgan_maqola),
        "Inli bola havolasi topilmadi. Topilmish matn:\n{}",
        korsatkich_bobi.content
    );

    // Chekinmani alohida tekshir
    assert!(
        korsatkich_bobi.content.contains("    * [REST API]"),
        "Bola atama 'REST API' notoʻgʻri, 4 boʻshliqli boʻlmagan chekinmaga ega"
    );
}
