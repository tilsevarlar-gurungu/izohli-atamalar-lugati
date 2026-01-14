use atama_umumiy::{Atama, UST_BELGILAR, atamalarni_yukla, ozbekcha_saralov_kalitini_ol};
use fancy_regex::{Captures, Regex};
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct AtamaTamgalovchi;

#[derive(Clone)]
struct MosAtama {
    atama: Atama,
    ocharjumla: String,
}

impl AtamaTamgalovchi {
    pub fn new() -> AtamaTamgalovchi {
        AtamaTamgalovchi
    }
    pub fn run_internal(
        &self,
        mut kitob: Book,
        terms_jild_asosi: Option<&Path>,
    ) -> mdbook::errors::Result<Book> {
        let terms_jildi = terms_jild_asosi
            .map(|yolak| yolak.to_path_buf())
            .unwrap_or_else(|| Path::new("src").join("terms"));
        let barcha_atamalar = atamalarni_yukla(&terms_jildi).unwrap_or_default();

        let mut moslar_biriktirmasi: HashMap<String, MosAtama> = HashMap::new();
        let mut sarlavha_kalitlari = Vec::new();
        let mut qisqartma_kalitlari = Vec::new();

        for atama in &barcha_atamalar {
            let umumiy_ocharjumla = format!("{} / {}", atama.inglizcha, atama.ruscha);

            let atama_kaliti = atama.sarlavha.to_lowercase();
            sarlavha_kalitlari.push(atama_kaliti.clone());
            moslar_biriktirmasi.insert(
                atama_kaliti,
                MosAtama {
                    atama: atama.clone(),
                    ocharjumla: umumiy_ocharjumla.clone(),
                },
            );

            if let Some(sifat) = &atama.sifat_shakli {
                let s_kalit = sifat.to_lowercase();
                sarlavha_kalitlari.push(s_kalit.clone());
                moslar_biriktirmasi.insert(
                    s_kalit,
                    MosAtama {
                        atama: atama.clone(),
                        ocharjumla: umumiy_ocharjumla.clone(),
                    },
                );
            }

            if let Some(qisqartma) = &atama.qisqartma {
                qisqartma_kalitlari.push(qisqartma.clone());
                moslar_biriktirmasi.insert(
                    qisqartma.clone(),
                    MosAtama {
                        atama: atama.clone(),
                        ocharjumla: umumiy_ocharjumla.clone(),
                    },
                );
            }
        }

        sarlavha_kalitlari.sort_by(|a, b| b.len().cmp(&a.len()));
        qisqartma_kalitlari.sort_by(|a, b| b.len().cmp(&a.len()));

        let qalqonlanmish_sarlavhalar = sarlavha_kalitlari
            .iter()
            .map(|k| regex::escape(k))
            .collect::<Vec<_>>()
            .join("|");
        let qalqonlanmish_qisqartmalar = qisqartma_kalitlari
            .iter()
            .map(|k| regex::escape(k))
            .collect::<Vec<_>>()
            .join("|");

        // --- Bosh qolipli ifoda ---
        // Birinchi guruh (Tashlab oʻtiladiganlar):
        // - ```[\s\S]*?```        -> bitik boʻlaklariga mos tushadi
        // - `[^`\n]+`             -> satrichi bitiklarga mos tushadi
        // - <abbr.*?>.*?</abbr>   -> mavjud tamgʻalarga mos tushadi
        // - \[.*?\]\(.*?\)        -> Markdown havolalariga mos tushadi
        // - <[^>]*>               -> HTML belgilariga mos tushadi
        let qolip_str = format!(
            r"(?i)(```[\s\S]*?```|`[^`\n]+`|<abbr\b[^>]*>[\s\S]*?</abbr>|\[[^\]]*\]\([^\)]*\)|<[^>]*>)|(\b(?:{})[a-zçşğö]*)|(?-i)(\b(?:{})\b)",
            qalqonlanmish_sarlavhalar, qalqonlanmish_qisqartmalar
        );

        let bosh_qolipli_ifoda = Regex::new(&qolip_str).expect("Bosh qolipli ifoda tuzilolmadi");
        let mut topilmish_fayl_nomlari = HashSet::new();

        kitob.for_each_mut(|unsur| {
            if let BookItem::Chapter(bob) = unsur {
                let atamadir = bob
                    .path
                    .as_ref()
                    .map_or(false, |yolak| yolak.starts_with("terms/"));
                if atamadir {
                    if let Some(nom) = bob.path.as_ref().and_then(|yolak| yolak.file_name()) {
                        topilmish_fayl_nomlari.insert(nom.to_owned());
                    }
                    bobni_ishlovdan_otkaz(bob, &bosh_qolipli_ifoda, &moslar_biriktirmasi);
                }
            }
        });

        if let Ok(maqolalar) = std::fs::read_dir(&terms_jildi) {
            for maqola in maqolalar.flatten() {
                let yolak = maqola.path();
                if yolak.extension().and_then(|s| s.to_str()) == Some("md") {
                    let fayl_nomi = yolak.file_name().unwrap();
                    if !topilmish_fayl_nomlari.contains(fayl_nomi) {
                        let matn = std::fs::read_to_string(&yolak).unwrap_or_default();
                        let nisbiy_yolak = PathBuf::from("terms").join(fayl_nomi);
                        let atama =
                            atama_umumiy::xotiradagi_atamani_yoy(&nisbiy_yolak, matn.clone());
                        let mut nomoddiy_bob =
                            Chapter::new(&atama.sarlavha, matn, nisbiy_yolak, Vec::new());
                        bobni_ishlovdan_otkaz(
                            &mut nomoddiy_bob,
                            &bosh_qolipli_ifoda,
                            &moslar_biriktirmasi,
                        );
                        kitob.sections.push(BookItem::Chapter(nomoddiy_bob));
                    }
                }
            }
        }
        Ok(kitob)
    }
}

impl Preprocessor for AtamaTamgalovchi {
    fn name(&self) -> &str {
        "atama-tamgalovchi"
    }

    fn run(&self, _vaziyat: &PreprocessorContext, kitob: Book) -> mdbook::errors::Result<Book> {
        self.run_internal(kitob, None)
    }
}

fn bobni_ishlovdan_otkaz(
    bob: &mut Chapter,
    qolipli_ifoda: &Regex,
    biriktirma: &HashMap<String, MosAtama>,
) {
    let matn = &bob.content;

    let mut ustbilgi_oxiri = 0;
    for satr in matn.lines().take(20) {
        if UST_BELGILAR.iter().any(|&b| satr.contains(b)) {
            if let Some(orin) = matn.find(satr) {
                ustbilgi_oxiri = orin + satr.len();
            }
        }
    }

    let h2_qi = Regex::new(r"(?m)^##\s+").unwrap();
    let adoq_boshi = h2_qi
        .find(matn)
        .ok()
        .flatten()
        .map(|m| m.start())
        .unwrap_or(matn.len());

    let ustbilgi = &matn[..ustbilgi_oxiri];
    let tana = &matn[ustbilgi_oxiri..adoq_boshi];
    let adoq = &matn[adoq_boshi..];

    let joriy_negiz = bob
        .path
        .as_ref()
        .and_then(|y| y.file_stem())
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let kuzatuvdagi_atamalar = std::cell::RefCell::new(HashMap::new());

    let tamgalanmish_tana = qolipli_ifoda
        .replace_all(tana, |ajratmalar: &Captures| {
            // 1 Guruh: Himoyalangan (bitik boʻlaklari va satrichi bitiklar)
            if let Some(tashlab_otilgan) = ajratmalar.get(1) {
                return tashlab_otilgan.as_str().to_string();
            }

            // 2 Guruh: Sarlavhalar (Katta-kichiklikka e'tiborsiz)
            if let Some(a) = ajratmalar.get(2) {
                let aj = a.as_str(); // ajratma jumlasi -> aj
                let mut kalit = aj.to_lowercase();
                while !kalit.is_empty() {
                    if let Some(berilmish) = biriktirma.get(&kalit) {
                        if is_self(joriy_negiz.as_str(), &berilmish.atama) {
                            return aj.to_string();
                        }
                        kuzatuvdagi_atamalar
                            .borrow_mut()
                            .insert(berilmish.atama.sarlavha.clone(), berilmish.atama.clone());
                        return format!(
                            "<abbr data-en-ru=\"{}\" aria-label=\"{}\">{}</abbr>",
                            berilmish.ocharjumla, berilmish.ocharjumla, aj
                        );
                    }
                    kalit.pop();
                }
            }

            // 3 Guruh: qisqartmalar (Katta-kichiklikka e'tiborli)
            if let Some(a) = ajratmalar.get(3) {
                let aj = a.as_str();
                if let Some(berilmish) = biriktirma.get(aj) {
                    if is_self(joriy_negiz.as_str(), &berilmish.atama) {
                        return aj.to_string();
                    }
                    kuzatuvdagi_atamalar
                        .borrow_mut()
                        .insert(berilmish.atama.sarlavha.clone(), berilmish.atama.clone());
                    return format!(
                        "<abbr data-en-ru=\"{}\" aria-label=\"{}\">{}</abbr>",
                        berilmish.ocharjumla, berilmish.ocharjumla, aj
                    );
                }
            }

            ajratmalar[0].to_string()
        })
        .into_owned();

    let mut oxirgi_matn = ustbilgi.to_string();
    oxirgi_matn.push_str(&tamgalanmish_tana);

    let topilmish = kuzatuvdagi_atamalar.into_inner();
    if !topilmish.is_empty() {
        let mut saralanmish: Vec<Atama> = topilmish.into_values().collect();
        saralanmish.sort_by(|a, b| {
            ozbekcha_saralov_kalitini_ol(&a.sarlavha)
                .cmp(&ozbekcha_saralov_kalitini_ol(&b.sarlavha))
        });
        oxirgi_matn.push_str("\n### Aloqali atamalar\n");
        for a in saralanmish {
            let fayl_nomi = a.yolak.file_name().unwrap().to_string_lossy();
            let mut satr = format!("- [{}]({})", a.sarlavha, fayl_nomi);
            if let Some(qisqartma) = &a.qisqartma {
                satr.push_str(&format!(" ({})", qisqartma));
            }
            satr.push_str(&format!(" — {} — {}\n", a.inglizcha, a.ruscha));
            oxirgi_matn.push_str(&satr);
        }
    }

    oxirgi_matn.push_str(adoq);
    bob.content = oxirgi_matn;
}

fn is_self(joriy_negiz: &str, atama: &Atama) -> bool {
    let atama_negizi = atama
        .yolak
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_lowercase();
    joriy_negiz == atama_negizi
}
