# Qöşuv

**Inglizça:** Merge<br>
**Rusça:** Слияние<br>
**Aloqali:** Git<br>
**Soha:** Dasturlov<br>

**Qöşuv** - bu ikki yoki undan ortiq bitik qöllarini bir-biriga birlaştiriş va ulardagi barça özgarişlarni yagona tarihga jamlaş jarayonidir. Odatda dasturçi yangi hususiyatni alohida qölda yozib bitirganidan söng, uni loyihaning ona qöliga qöşuv amali orqali birlaştiradi.

Bilimsanar fanida qöşuv amali ikki hil yölda bölişi mumkin: agar ona qölda heç qanday özgariş bölmagan bölsa, Git şunçaki qaydnomani oldinga surib qöyadi (fast-forward). Agar har ikkala qölda ham yangi qaydlar mavjud bölsa, Git yangi qöşuv qaydini yaratadi. Bu jarayonda agar bir hil satrlar har hil özgartirilgan bölsa, töqnaşuv yuzaga keladi va uni qölda hal qiliş talab etiladi.

```
(yangi-hususiyat)
           A ------ B
          /          \
--- C --- D --- E --- F (ona-qöl)
                      ^
                [Qöşuv qaydi]
```

## Etiborga olingan muqobillar

- birlaştiriş
