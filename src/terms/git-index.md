# Qaydnoma

**Inglizça:** Index<br>
**Rusça:** Индекс<br>
**Aloqali:** Git<br>
**Soha:** Dasturlov<br>

**Qaydnoma** — bu bitikdagi özgarişlar va fayllarning navbatdagi qayd uçun tayyorlab qöyilgan holatini saqlovçi mahsus malumotlar omboridir. Bilimsanar fanida u köpinça qayd maydoni bilan bir hil manoda işlatiladi, biroq tehnik jihatdan qaydnoma — bu Gitning içki qismidagi bir "fayl" (odatda .git/index) bölib, u tahrirdagi oğoç va loyiha tarihi orasidagi köprik vazifasini ötaydi.

Dasturçi `git add` buyruğini berganda, Git faylning nushasini emas, balki uning öşa paytdagi holati haqidagi malumotlarni aynan şu qaydnomaga yozib qöyadi. Qaydnoma bitikning "kelajakdagi nushasi" qanday bölişini belgilaydi. Şuning uçun, hatto siz diskdagi faylni öçirib yuborsangiz ham, agar u qaydnomada bölsa, Git uni qayd qilişga tayyor deb hisoblayveradi.

```
[Tahrirdagi oğoç]  ------>  [Qaydnoma (Index)]  ------>  [Dafina]
  (Diskdagi fayl)            (Tayyorlangan holat)         (Muhrlangan tarih)
```

## Etiborga olingan muqobillar

- körsatkiç
