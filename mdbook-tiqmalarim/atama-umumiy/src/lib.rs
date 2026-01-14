use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Atama {
    pub sarlavha: String,
    pub yolak: PathBuf,
    pub matn: String,
    pub inglizcha: String,
    pub ruscha: String,
    pub ota: Option<String>,
    pub aloqali: Option<String>,
    pub soha: Option<String>,
    pub qisqartma: Option<String>,
    pub sifat_shakli: Option<String>,
}

pub const UST_BELGILAR: &[&str] = &[
    "**Inglizça:**",
    "**Rusça:**",
    "**Ota-atama:**",
    "**Aloqali:**",
    "**Soha:**",
    "**Qisqartma:**",
    "**Sifat şakli:**",
];

pub fn xotiradagi_atamani_yoy(yolak: &Path, matn: String) -> Atama {
    let mut sarlavha = yolak.file_stem().unwrap().to_string_lossy().into_owned();
    let mut inglizcha = String::from("N/A");
    let mut ruscha = String::from("N/A");
    let mut ota = None;
    let mut aloqali = None;
    let mut soha = None;
    let mut qisqartma = None;
    let mut sifat_shakli = None;
    let mut sarlavha_topildi = false;

    for satr in matn.lines().take(25) {
        let taroshlanmish = satr.trim();
        if !sarlavha_topildi && taroshlanmish.starts_with("# ") {
            sarlavha = taroshlanmish[2..].trim().to_string();
            sarlavha_topildi = true;
        } else if taroshlanmish.contains("**Inglizça:**") {
            inglizcha = qiymatni_chiqarib_ol(taroshlanmish);
        } else if taroshlanmish.contains("**Rusça:**") {
            ruscha = qiymatni_chiqarib_ol(taroshlanmish);
        } else if taroshlanmish.contains("**Ota-atama:**") {
            let oa = qiymatni_chiqarib_ol(taroshlanmish);
            if !oa.is_empty() && oa != "Yöq" && oa != "None" {
                ota = Some(oa);
            }
        } else if taroshlanmish.contains("**Aloqali:**") {
            let al = qiymatni_chiqarib_ol(taroshlanmish);
            if !al.is_empty() {
                aloqali = Some(al);
            }
        } else if taroshlanmish.contains("**Soha:**") {
            let s = qiymatni_chiqarib_ol(taroshlanmish);
            if !s.is_empty() {
                soha = Some(s);
            }
        } else if taroshlanmish.contains("**Qisqartma:**") {
            let q = qiymatni_chiqarib_ol(taroshlanmish);
            if !q.is_empty() {
                qisqartma = Some(q);
            }
        } else if taroshlanmish.contains("**Sifat şakli:**") {
            let ss = qiymatni_chiqarib_ol(taroshlanmish);
            if !ss.is_empty() {
                sifat_shakli = Some(ss);
            }
        }
    }

    Atama {
        sarlavha,
        yolak: yolak.to_path_buf(),
        matn,
        inglizcha,
        ruscha,
        ota,
        aloqali,
        soha,
        qisqartma,
        sifat_shakli,
    }
}

pub fn atamalarni_yukla(terms_jildi: &Path) -> io::Result<Vec<Atama>> {
    let mut barcha_atamalar = Vec::new();
    if let Ok(maqolalar) = fs::read_dir(terms_jildi) {
        for maqola in maqolalar.flatten() {
            let yolak = maqola.path();
            if yolak.extension().and_then(|s| s.to_str()) == Some("md") {
                let matn = fs::read_to_string(&yolak).unwrap_or_default();
                let nisbiy_yolak = PathBuf::from("terms").join(yolak.file_name().unwrap());
                barcha_atamalar.push(xotiradagi_atamani_yoy(&nisbiy_yolak, matn));
            }
        }
    }
    atamalarni_sarala(&mut barcha_atamalar);
    Ok(barcha_atamalar)
}

pub fn atamalarni_sarala(atamalar: &mut Vec<Atama>) {
    atamalar.sort_by(|a, b| a.sarlavha.cmp(&b.sarlavha));
}

fn qiymatni_chiqarib_ol(satr: &str) -> String {
    // Find where the label ends
    if let Some(pos) = satr.find(":**") {
        let value = &satr[pos + 3..]; // Move past the ":**"
        return value
            .replace("<br>", "")
            .replace("<br/>", "")
            .trim()
            .to_string();
    }
    String::new()
}

pub fn langar_tanitkichini_ol(harf: &str) -> String {
    harf.to_lowercase()
        .replace("ö", "o-uz")
        .replace("ğ", "g-uz")
        .replace("ş", "sh-uz")
        .replace("ç", "ch-uz")
        .replace("ng", "ng-uz")
}

pub fn ozbekcha_bosh_qismni_ol(sarlavha: &str) -> String {
    let s = sarlavha.to_lowercase();
    s.chars().next().unwrap_or('#').to_uppercase().to_string()
}

pub fn ozbekcha_saralov_kalitini_ol(sarlavha: &str) -> Vec<usize> {
    let alifbo = vec![
        "a", "b", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
        "t", "u", "v", "x", "y", "z", "ö", "ğ", "ş", "ç", "ng",
    ];
    let mut kalit = Vec::new();
    let belgilar_vec: Vec<char> = sarlavha.chars().collect();
    let mut i = 0;
    while i < belgilar_vec.len() {
        let mut mos_keldi = false;
        if i + 1 < belgilar_vec.len() {
            let qoshma_belgi = format!("{}{}", belgilar_vec[i], belgilar_vec[i + 1]);
            if let Some(orin) = alifbo.iter().position(|&x| x == qoshma_belgi) {
                kalit.push(orin);
                i += 2;
                mos_keldi = true;
            }
        }
        if !mos_keldi {
            let belgi = belgilar_vec[i].to_string();
            kalit.push(alifbo.iter().position(|&x| x == belgi).unwrap_or(99));
            i += 1;
        }
    }
    kalit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sina_yoyiluvni() {
        let matn = r#"# Mening Atamam
**Inglizça:** My Term
**Rusça:** Мой Термин
**Sifat şakli:** sifatli"#;

        let atama = xotiradagi_atamani_yoy(Path::new("terms/test.md"), matn.to_string());

        assert_eq!(atama.sarlavha, "Mening Atamam");
        assert_eq!(atama.inglizcha, "My Term");
        assert_eq!(atama.sifat_shakli, Some("sifatli".to_string()));
    }
}
