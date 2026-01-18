# Bekorlov

**Inglizça:** Revert<br>
**Rusça:** Отмена изменений<br>
**Aloqali:** Git<br>
**Soha:** Dasturlov<br>

**Bekorlov** - bu loyiha tarihidagi malum bir qaydni unga teskari bölgan yangi qayd yaratiş orqali bekor qiliş amalidir. Qaytaruvdan farqli ölaroq, bekorlov loyiha tarihini öçirib taşlamaydi, balki hatoni tuzatiş faktini ham tarihga muhrlab qöyadi.

Bilimsanar fanida bu usul "havfsiz bekor qiliş" deb hisoblanadi, ayniqsa jamoaviy işlaşda. Agar siz allaqaçon olis dafinaga yuborilgan qaydni öçirmoqçi bölsangiz, qaytaruv işlatiş boşqa dasturçilarning işiga zarar yetkazişi mumkin. Bekorlov esa şunçaki "oldingi özgarişni teskarisiga aylantiruvçi" yangi qayd qöşadi, natijada loyiha tarihi buzilmaydi va hammaning bilimsanarida muvozanat saqlanib qoladi.

```bash
# Holat:
[Qayd A] --- [Qayd B (Hatoli bitik)] --- [Qayd C]

$ git revert [Qayd B]

# Natija:
[Qayd A] --- [Qayd B] --- [Qayd C] --- [Qayd D (B-ning teskarisi)]
# Tarih saqlandi, lekin B-dagi hatolar D-da bekor qilindi.
```

## Etiborga olingan muqobillar

- bekor qiluv
- qaytuv
- öçiruv
