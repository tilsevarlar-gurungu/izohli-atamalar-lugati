use atama_umumiy::atamalarni_yukla;
use git2::{Repository, Sort};
use mdbook::book::{Book, BookItem};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct KirishOldIshlovchisi;

impl KirishOldIshlovchisi {
    pub fn new() -> Self {
        Self
    }

    pub fn run_internal(&self, mut kitob: Book, atamalar_jildi: &Path) -> anyhow::Result<Book> {
        // 1. Atamalarni yuklaymiz (bu bizga terms/atama.md ni beradi)
        let barcha_atamalar = atamalarni_yukla(atamalar_jildi)?;

        // 2. Git Dafinasi yorligʻini yaratamiz
        let dafina = Repository::discover(atamalar_jildi).ok();
        let dafina_ildizi = dafina.as_ref().and_then(|r| r.workdir()).map(PathBuf::from);

        let mut maqolalar = Vec::new();

        for atama in &barcha_atamalar {
            // mutlaq yoʻlak  "src/terms/terms/file.md" kabi inlagan yoʻlak
            // xatolarining oldini olishga yordam beradi.
            let mutlaq_yolak = if atama.yolak.is_absolute() {
                atama.yolak.clone()
            } else {
                let nomzod = atamalar_jildi.join(atama.yolak.file_name().unwrap());
                if nomzod.exists() {
                    nomzod
                } else {
                    // ehtiyot yuzasidan 'terms' ning otasi bilan qoshish (odatda 'src')
                    atamalar_jildi
                        .parent()
                        .unwrap_or(Path::new(""))
                        .join(&atama.yolak)
                }
            };

            // Agar fayl boʻlmasa uni tashlab oʻtamiz
            if !mutlaq_yolak.exists() {
                continue;
            }

            let mut qayd_sanalari = Vec::new();

            // 3. Git tarixini chiqarib olamiz
            if let (Some(dafina), Some(ildiz)) = (&dafina, &dafina_ildizi) {
                if let Ok(nisbiy_yolak) = mutlaq_yolak.strip_prefix(ildiz) {
                    let git_yolagi_str = nisbiy_yolak.to_string_lossy().replace("\\", "/");

                    if let Ok(mut qayd_ziyorati) = dafina.revwalk() {
                        let _ = qayd_ziyorati.set_sorting(Sort::TIME);
                        let _ = qayd_ziyorati.push_head();

                        for oid in qayd_ziyorati.flatten() {
                            if let Ok(qayd) = dafina.find_commit(oid) {
                                if let Ok(ogoch) = qayd.tree() {
                                    if ogoch.get_path(Path::new(&git_yolagi_str)).is_ok() {
                                        qayd_sanalari.push(qayd.time().seconds());
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 4. Ustbilgi uchun muqobil
            // (agar Git hech qanday tarixga ega boʻlmaganda ishlaydi)
            let (qoshilmish, yangilanmish) = if qayd_sanalari.is_empty() {
                let ustbilgi = std::fs::metadata(&mutlaq_yolak)?;
                let q = ustbilgi
                    .created()
                    .unwrap_or_else(|_| ustbilgi.modified().unwrap())
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs() as i64;
                let y = ustbilgi
                    .modified()?
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs() as i64;
                (q, y)
            } else {
                (
                    *qayd_sanalari.iter().min().unwrap(),
                    *qayd_sanalari.iter().max().unwrap(),
                )
            };

            maqolalar.push((
                atama.sarlavha.clone(),
                mutlaq_yolak,
                qoshilmish,
                yangilanmish,
            ));
        }

        // 5. Saralash va Elash
        // Oxirgi Qoʻshilgan Atamalar (oxirgi 5 ta)
        maqolalar.sort_by(|a, b| b.2.cmp(&a.2));
        let oxirgi_qoshilganlar: Vec<_> = maqolalar.iter().take(5).cloned().collect();
        let qoshilmish_yolaklari: HashSet<_> =
            oxirgi_qoshilganlar.iter().map(|e| e.1.clone()).collect();

        // Oxirgi Yangilangan Atamalar (oxirgi 5 ta)
        let mut qolganlari: Vec<_> = maqolalar
            .into_iter()
            .filter(|e| !qoshilmish_yolaklari.contains(&e.1))
            .collect();
        qolganlari.sort_by(|a, b| b.3.cmp(&a.3));
        let oxirgi_yangilanganlar = qolganlari.into_iter().take(5);

        // 6. Markdown roʻyxatini tuzish
        let mut qoshilmish_qiymati = String::new();
        for (sarlavha, yolak, _, _) in oxirgi_qoshilganlar {
            let fayl_negizi = yolak.file_stem().unwrap().to_string_lossy();
            qoshilmish_qiymati.push_str(&format!("* [{sarlavha}](terms/{fayl_negizi}.html)\n"));
        }

        let mut yangilanmish_qiymati = String::new();
        for (sarlavha, yolak, _, _) in oxirgi_yangilanganlar {
            let fayl_negizi = yolak.file_stem().unwrap().to_string_lossy();
            yangilanmish_qiymati.push_str(&format!("* [{sarlavha}](terms/{fayl_negizi}.html)\n"));
        }

        // 7. intro.md'ga kiritamiz
        kitob.for_each_mut(|unsur| {
            if let BookItem::Chapter(bob) = unsur {
                // (|| ch.name == "Kirish") kabi muqobil yoʻlni tekshirish
                // yasama bob boʻsh yoʻlakka ega boʻlsa ham
                // sinovlarimizning muvaffaqiyatli oʻtishini ta'minlaydi
                let kirishdir = bob.path.as_ref().map_or(false, |yolak| {
                    let yolak_str = yolak.to_string_lossy().to_lowercase();
                    yolak_str == "intro.md"
                        || yolak_str.ends_with("/intro.md")
                        || yolak_str.ends_with("\\intro.md")
                }) || bob.name == "Kirish";

                if kirishdir {
                    if bob.content.contains("{{jamiAtamalar}}") {
                        bob.content = bob
                            .content
                            .replace("{{jamiAtamalar}}", &barcha_atamalar.len().to_string());
                    }
                    if bob.content.contains("{{oxirgiQoshilganlar}}") {
                        bob.content = bob
                            .content
                            .replace("{{oxirgiQoshilganlar}}", qoshilmish_qiymati.trim_end());
                    }
                    if bob.content.contains("{{oxirgiYangilanganlar}}") {
                        bob.content = bob
                            .content
                            .replace("{{oxirgiYangilanganlar}}", yangilanmish_qiymati.trim_end());
                    }
                }
            }
        });

        Ok(kitob)
    }
}
