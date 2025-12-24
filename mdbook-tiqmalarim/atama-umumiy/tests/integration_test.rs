use atama_umumiy::{Atama, xotiradagi_atamani_yoy};
use std::path::Path;

#[test]
fn sina_alxorazma_misolining_yoyuvini() {
    // 1. Namuna ma'lumotni xuddi berilganiday tayyorla
    let matn = r#"# Alxorazma

**Inglizcha:** Algorithm<br>
**Ruscha:** Алгоритм<br>
**Soha:** Dasturlov<br>
**Ota-atama:** Bilimsanar

**Alxorazma** – bu aniq bir muammoni hal qilish yoki hisob-kitobni amalga
oshirish uchun moʻljallangan.

## E’tiborga olingan muqobillar

- algoritm
- alxorizm"#;

    let yolak = Path::new("terms/alxorazma.md");

    // 2. Yoʻygichni ishga sol
    let atama = xotiradagi_atamani_yoy(yolak, matn.to_string());

    // 3. Tasdiqlovlar

    // Asosiy Tanitkichni tekshir
    assert_eq!(atama.sarlavha, "Alxorazma");
    assert_eq!(atama.yolak.to_str().unwrap(), "terms/alxorazma.md");

    // Ustbilgi tozaligini tekshir (<br> ning oʻchirilganiga
    // va taroshlashning muvaffaqiyatli amalga oshganini tekshir
    assert_eq!(atama.inglizcha, "Algorithm");
    assert_eq!(atama.ruscha, "Алгоритм");
    assert_eq!(atama.soha, Some("Dasturlov".to_string()));
    assert_eq!(atama.ota, Some("Bilimsanar".to_string()));

    // Matnning asliday saqlanganligini tekshir
    assert!(atama.matn.contains("## E’tiborga olingan muqobillar"));
    assert!(atama.matn.contains("- alxorizm"));
}

#[test]
fn sina_bazi_ixtiyoriy_qiymatlari_bolmagan_malumot_yoyuvini() {
    // Soha, Ota-atama, yoki Sifat shakli qiymatlari boʻlmagan
    // atamani tekshir
    let matn = r#"# Minimal
**Inglizcha:** Min
**Ruscha:** Мин"#;

    let yolak = Path::new("terms/min.md");
    let atama = xotiradagi_atamani_yoy(yolak, matn.to_string());

    assert_eq!(atama.sarlavha, "Minimal");
    assert_eq!(atama.soha, None);
    assert_eq!(atama.ota, None);
    assert_eq!(atama.sifat_shakli, None);
}

#[test]
fn sina_ozbekcha_saralov_kirishimini() {
    use atama_umumiy::atamalarni_sarala;

    let mut atamalar = vec![
        toqima_atama_yarat("Sabzi"),
        toqima_atama_yarat("Oʻrdak"),
        toqima_atama_yarat("Shamol"),
        toqima_atama_yarat("Olma"),
    ];

    atamalarni_sarala(&mut atamalar);

    // Rasmiy tartib: A-Z, keyin Oʻ, G', Sh, Ch, Ng
    // 1. Olma (O 13-oʻrinda)
    // 2. Sabzi (S 17-oʻrinda )
    // 3. Oʻrdak (Oʻ 24-oʻrinda)
    // 4. Shamol (Sh 26-oʻrinda)

    assert_eq!(atamalar[0].sarlavha, "Olma");
    assert_eq!(atamalar[1].sarlavha, "Sabzi");
    assert_eq!(atamalar[2].sarlavha, "Oʻrdak");
    assert_eq!(atamalar[3].sarlavha, "Shamol");
}

// saralov sinovlari uchun yordamchi topshiriq
fn toqima_atama_yarat(sarlavha: &str) -> Atama {
    xotiradagi_atamani_yoy(
        Path::new("test.md"),
        format!("# {}\n**Inglizcha:** N/A\n**Ruscha:** N/A", sarlavha),
    )
}
