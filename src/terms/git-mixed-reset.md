# Yarim qaytaruv

**Inglizça:** Mixed Reset<br>
**Rusça:** Смешанный сброс<br>
**Aloqali:** Git<br>
**Soha:** Dasturlov<br>

**Yarim qaytaruv** — bu ham loyiha tarihidagi körsatkiçni (Boş / HEAD) orqaga suruvçi, ham Qaydnomani tozalovçi amaldir. Bu tartib "yarim" deb ataladi, çunki u Git'ning içki hotirasini (tarih va savatçani) özgartiradi, lekin sizning Tahrirdagi oğoçingizdagi (diskdagi) fayllaringizga tegmaydi.

Dasturçi uçun bu tartib şunday işlaydi: siz qaydni bekor qilasiz, savatçadagi ("git add" qilingan) fayllar u yerdan çiqib ketadi, lekin yozgan bitiklaringiz diskda öz holiça turadi. Bu odatda bir neçta fayllarni "git add" qilib yuborganingizdan söng, ularni qayta saralaş yoki ayrimlarini qaydga qöşmaslikka qaror qilganingizda juda qöl keladi.

```bash
# Jarayon:
1. [Qayd A] -> [Qayd B] (Hozirgi holat)
2. $ git reset --mixed HEAD~1  # yoki şunçaki 'git reset HEAD~1'
3. [Qayd A] (Yangi HEAD)
   * Qaydnoma (Index) böşadi.
   * Fayllar diskda özgartirilgan holatda turibdi.
```

## Etiborga olingan muqobillar

- aralaş qaytaruv
- çala qaytaruv
