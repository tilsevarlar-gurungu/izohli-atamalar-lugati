use atama_umumiy::{
    Atama, atamalarni_sarala, langar_tanitkichini_ol, ozbekcha_bosh_qismni_ol,
    xotiradagi_atamani_yoy,
};
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use std::collections::HashSet;
use std::path::Path;

pub struct AtamaOrinlovchi;

impl AtamaOrinlovchi {
    pub fn new() -> AtamaOrinlovchi {
        AtamaOrinlovchi
    }
    pub fn run_internal(&self, mut kitob: Book, terms_jild_asosi: Option<&Path>) -> Result<Book> {
        let mut barcha_atamalar = Vec::new();
        let mut topilmish_fayl_nomlari = HashSet::new();
        let mut summaryda_yoq = Vec::new();

        // 1. xotiradan yigʻib ol
        kitob.for_each_mut(|unsur| {
            if let BookItem::Chapter(bob) = unsur {
                if let Some(ref yolak) = bob.path {
                    if yolak.starts_with("terms/") {
                        let atama = xotiradagi_atamani_yoy(yolak, bob.content.clone());
                        barcha_atamalar.push(atama);
                        if let Some(nom) = yolak.file_name() {
                            topilmish_fayl_nomlari.insert(nom.to_owned());
                        }
                    }
                }
            }
        });

        // 2. Oʻchmas xotiradan yigʻib ol
        // sinovlar uchun toʻqima qiymatlardan foydalan
        // agar ular boʻlmasa fitriy qiymatlarni ishlat
        let terms_jildi = terms_jild_asosi
            .map(|yolak| yolak.to_path_buf())
            .unwrap_or_else(|| Path::new("src").join("terms"));
        if let Ok(maqolalar) = std::fs::read_dir(terms_jildi) {
            for maqola in maqolalar.flatten() {
                let maqola_yolagi = maqola.path();
                if maqola_yolagi.extension().and_then(|s| s.to_str()) == Some("md") {
                    let maqola_fayl_nomi = maqola_yolagi.file_name().unwrap();
                    if !topilmish_fayl_nomlari.contains(maqola_fayl_nomi) {
                        let matn = std::fs::read_to_string(&maqola_yolagi).unwrap_or_default();
                        let nisbiy_yolak = Path::new("terms").join(maqola_fayl_nomi);
                        let atama = xotiradagi_atamani_yoy(&nisbiy_yolak, matn);

                        barcha_atamalar.push(atama.clone());
                        summaryda_yoq.push(atama);
                    }
                }
            }
        }

        atamalarni_sarala(&mut barcha_atamalar);

        // 3. Koʻrsatkich MARKDOWN matnini qur
        let mut korsatkich_md =
            String::from("# Özbekça - Inglizça - Rusça Körsatkiç\n\n<div id=\"top\"></div>\n\n");

        let alifbo_tartibi = vec![
            "A", "B", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
            "S", "T", "U", "V", "X", "Y", "Z", "Ö", "Ğ", "Ş", "Ç", "Ng",
        ];

        let mut yoltanim_qatori = String::from("<p align=\"center\" style=\"line-height: 3;\">\n");
        for harf in &alifbo_tartibi {
            if barcha_atamalar
                .iter()
                .any(|t| &ozbekcha_bosh_qismni_ol(&t.sarlavha) == harf)
            {
                let langar = langar_tanitkichini_ol(harf);
                yoltanim_qatori.push_str(&format!(
                    "<a href=\"#{}\" style=\"text-decoration: none; border: 1px solid #ccc; padding: 4px 8px; border-radius: 4px; font-weight: bold; font-size: 0.9em;\">{}</a> &nbsp;\n",
                    langar, harf
                ));
            }
        }
        yoltanim_qatori.push_str("</p>\n\n---\n\n");
        korsatkich_md.push_str(&yoltanim_qatori);

        for harf in alifbo_tartibi {
            let guruhdagi_atamalar: Vec<&Atama> = barcha_atamalar
                .iter()
                .filter(|a| ozbekcha_bosh_qismni_ol(&a.sarlavha) == harf)
                .collect();

            if !guruhdagi_atamalar.is_empty() {
                let langar = langar_tanitkichini_ol(harf);
                korsatkich_md.push_str(&format!("## <span id=\"{}\"></span>{}\n", langar, harf));

                for atama in guruhdagi_atamalar {
                    let mut satr = format!(
                        "- [**{}**]({})",
                        atama.sarlavha,
                        atama.yolak.to_string_lossy()
                    );

                    if let Some(qisqartma) = &atama.qisqartma {
                        satr.push_str(&format!(" ({})", qisqartma));
                    }

                    if let Some(s) = &atama.soha {
                        satr.push_str(&format!(" <small><i>[{}]</i></small>", s));
                    }

                    if let Some(aloqali_atama_nomi) = &atama.aloqali {
                        if let Some(aloqali_atama) = barcha_atamalar.iter().find(|aa| {
                            aa.sarlavha.to_lowercase() == aloqali_atama_nomi.to_lowercase()
                        }) {
                            satr.push_str(&format!(
                                ", *[{}]({})*",
                                aloqali_atama.sarlavha,
                                aloqali_atama.yolak.to_string_lossy()
                            ));
                        } else {
                            satr.push_str(&format!(", *{}*", aloqali_atama_nomi));
                        }
                    }

                    satr.push_str(&format!(" — {} — {}\n", atama.inglizcha, atama.ruscha));
                    korsatkich_md.push_str(&satr);

                    if let Some(ota_nomi) = &atama.ota {
                        if let Some(ota_atama) = barcha_atamalar
                            .iter()
                            .find(|a| a.sarlavha.to_lowercase() == ota_nomi.to_lowercase())
                        {
                            korsatkich_md.push_str(&format!(
                                "    * [**{}**]({})\n",
                                ota_atama.sarlavha,
                                ota_atama.yolak.to_string_lossy()
                            ));
                        }
                    }

                    for bola in &barcha_atamalar {
                        if let Some(o) = &bola.ota {
                            if o.trim().to_lowercase() == atama.sarlavha.trim().to_lowercase() {
                                korsatkich_md.push_str(&format!(
                                    "    * [{}]({})\n",
                                    bola.sarlavha,
                                    bola.yolak.to_string_lossy()
                                ));
                            }
                        }
                    }
                }
                korsatkich_md.push_str("\n<div align=\"right\"><a href=\"#top\" style=\"text-decoration: none; color: #aaa; font-size: 0.9em;\">↑</a></div>\n\n");
            }
        }

        // 4. Oʻrin egallab turuvchilarni haqiqiy matn bilan almashtir
        let mut topilmish_orin_ushlab_turuvchilar = false;

        kitob.for_each_mut(|unsur| {
            if let BookItem::Chapter(bob) = unsur {
                let atama_hujjatidir = bob
                    .path
                    .as_ref()
                    .map(|yolak| yolak.file_name().map_or(false, |os| os == "terms.md"))
                    .unwrap_or(false);

                if atama_hujjatidir {
                    bob.content = korsatkich_md.clone();
                    bob.name = "Özbekça - Inglizça - Rusça Körsatkiç".to_string();
                    topilmish_orin_ushlab_turuvchilar = true;
                }
            }
        });

        if !topilmish_orin_ushlab_turuvchilar {
            let korsatkich_bobi = Chapter::new(
                "Özbekça - Inglizça - Rusça Körsatkiç",
                korsatkich_md,
                Path::new("terms.md"),
                Vec::new(),
            );
            kitob.sections.insert(0, BookItem::Chapter(korsatkich_bobi));
        }

        // 5. Moslashuvchan atama sahifalarini kirgiz
        for atama in summaryda_yoq {
            let yangi_bob = Chapter::new(
                &atama.sarlavha, // Fixed: uses Atama Title instead of filename for HTML <title>
                atama.matn,
                atama.yolak,
                Vec::new(),
            );
            kitob.sections.push(BookItem::Chapter(yangi_bob));
        }

        Ok(kitob)
    }
}

impl Preprocessor for AtamaOrinlovchi {
    fn name(&self) -> &str {
        "atama-orinlovchi"
    }

    fn run(&self, _vaziyat: &PreprocessorContext, kitob: Book) -> Result<Book> {
        self.run_internal(kitob, None)
    }
}
