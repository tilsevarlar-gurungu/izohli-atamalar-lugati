# Quruq qaytaruv

**Inglizça:** Soft Reset<br>
**Rusça:** Мягкий сброс<br>
**Aloqali:** Git<br>
**Soha:** Dasturlov<br>

**Quruq qaytaruv** - bu loyiha tarihidagi körsatkiçni (HEAD) orqaga suriş, ammo kiritilgan özgarişlarni butunlay öçirmasdan, ularni qayd maydonida saqlab qoliş amalidir. "Quruq" deb atalişiga sabab — bu amal fayllarning mazmuniga yoki tayyorlangan özgarişlarga zarar yetkazmaydi, faqatgina ohirgi qaydnomani "yeçib" yuboradi.

Bilimsanar fanida quruq qaytaruv köp hollarda ohirgi qilingan qaydni tahrirlaş yoki bir neça kiçik qaydlarni bitta yirik qaydga birlaştiriş uçun işlatiladi. Masalan, siz qaydnomani muhrladingiz, lekin unga biror faylni qöşişni unütganingizni sezib qoldingiz. Quruq qaytaruv orqali siz qaydnomani bekor qilasiz, yangi faylni qöşasiz va hammasini qaytadan bitta toza qayd qilib muhrlaysiz.

```
[Tarih]: Qayd A ---> Qayd B (HEAD)
                          |
      $ git reset --soft [Qayd A]
                          |
[Natija]: Qayd A (HEAD) + (Qayd B'dagi özgarişlar hali ham savatçada/indexda)
```

## Etiborga olingan muqobillar

- böş qaytaruv
- yumşoq qaytaruv
