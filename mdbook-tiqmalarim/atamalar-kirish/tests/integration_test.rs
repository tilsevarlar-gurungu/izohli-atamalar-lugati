use atamalar_kirish::KirishOldIshlovchisi;
use mdbook::book::{Book, BookItem, Chapter};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[test]
fn sina_kirish_ozgardimi() {
    let muvaqqat_jild = tempfile::tempdir().unwrap();
    let terms_yolagi = muvaqqat_jild.path().join("terms");
    fs::create_dir(&terms_yolagi).unwrap();

    // 1. Turli vaqttamgʻali atamalar qoʻshamiz
    // "old.md" birinchi qoʻshiladi
    fs::write(
        terms_yolagi.join("old.md"),
        "# Eski\n**Inglizça:** Old\n**Rusça:** Старый",
    )
    .unwrap();

    thread::sleep(Duration::from_millis(100));

    // "new.md" ikkinchi boʻlib qoʻshiladi (u oxirgiQoshilganlar roʻyxatida boʻladi)
    fs::write(
        terms_yolagi.join("new.md"),
        "# Yangi\n**Inglizça:** New\n**Rusça:** Новый",
    )
    .unwrap();

    // 2. ichida oʻrin ushlab turuvchilari mavjud intro.md hujjatli yasama kitob yarating
    let mut kitob = Book::new();
    let kirish_matni =
        "Jami: {{jamiAtamalar}}\nYangi: {{oxirgiQoshilganlar}}\nYangilangan: {{oxirgiYangilanganlar}}";

    kitob.sections.push(BookItem::Chapter(Chapter::new(
        "Introduction",
        kirish_matni.to_string(),
        PathBuf::from("intro.md"),
        vec![],
    )));

    // 3. Oldishlovchini yurgizing
    let oldishlovchi = KirishOldIshlovchisi::new();
    let natija = oldishlovchi.run_internal(kitob, &terms_yolagi).unwrap();

    // 4. Natijalarni tekshiring
    let kirish_bobi = match &natija.sections[0] {
        BookItem::Chapter(bob) => bob,
        _ => panic!("Kiriş bobi topilmadi"),
    };

    assert!(kirish_bobi.content.contains("Jami: 2"));

    // "Yangi" va "Eski" Qoʻshilganlar roʻyxatida boʻlishi kerak chunki
    // hozirgacha faqat 2 ta fayl mavjud va bitigimiz oxirgi 5 tani ajratib oladi
    assert!(kirish_bobi.content.contains("* [Yangi](terms/new.html)"));
    assert!(kirish_bobi.content.contains("* [Eski](terms/old.html)"));
}

#[test]
fn sina_kirish_orin_ushlab_turuvchilar_almashtirildimi() {
    let muvaqqat_jild = tempfile::tempdir().unwrap();
    let terms_yolagi = muvaqqat_jild.path().join("terms");
    fs::create_dir(&terms_yolagi).unwrap();

    fs::write(
        terms_yolagi.join("test1.md"),
        "# Bir\n**Inglizça:** One\n**Rusça:** Один",
    )
    .unwrap();

    let mut kitob = Book::new();
    // Roʻyxat uchun maxsus oʻrin ushlab turuvchi qoʻsh
    let kirish_matni = "Soni: {{jamiAtamalar}}\nRöyxat:\n{{oxirgiQoshilganlar}}";
    kitob.sections.push(BookItem::Chapter(Chapter::new(
        "Kiriş",
        kirish_matni.to_string(),
        PathBuf::from("intro.md"),
        vec![],
    )));

    let oldishlovchi = KirishOldIshlovchisi::new();
    let natija = oldishlovchi.run_internal(kitob, &terms_yolagi).unwrap();

    let kirish_bobi = match &natija.sections[0] {
        BookItem::Chapter(bob) => bob,
        _ => panic!(),
    };

    // Almashtirilgan oʻrin ushlab turuvchilarni tekshir
    assert!(kirish_bobi.content.contains("Soni: 1"));
    assert!(kirish_bobi.content.contains("* [Bir](terms/test1.html)"));
    assert!(!kirish_bobi.content.contains("{{oxirgiQoshilganlar}}"));
}
