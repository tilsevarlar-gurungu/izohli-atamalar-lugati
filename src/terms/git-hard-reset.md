# Töla qaytaruv

**Inglizça:** Hard Reset<br>
**Rusça:** Жесткий сброс<br>
**Aloqali:** Git<br>
**Soha:** Dasturlov<br>

**Töla qaytaruv** - bu Git tizimidagi eng keskin va qaytarib bölmaydigan buyruqlardan biridir. U ham loyiha tarihidagi körsatkiçni (Boş / HEAD), ham Qaydnomani, ham sizning Tahrirdagi oğoçingizni (diskdagi fayllarni) tanlangan nuqtaga majburan qaytaradi.

Bilimsanar fanida bu amal "hamma narsani tozalaş va orqaga qaytariş" deb tüşuniladi. Töla qaytaruvdan söng, belgilangan qayddan keyin qilingan barça saqlanmagan özgarişlar va yangi yozilgan bitiklar butunlay öçirib yuboriladi. Dasturçilar bu buyruqni asosan bitikda biron-bir tajriba ötkazib, u muvaffaqiyatsiz tugaganida va loyihani tezda "toza va işçi" holatiga keltiriş kerak bölganda işlatişadi.

> Ogohlantiriş: Töla qaytaruvni işlatişdan oldin ehtiyot böling, çunki hali qayd qilinmagan barça mehnatlaringizni tiklab bölmas darajada yöqotişingiz mumkin.

```
# Holat:
1. [Qayd A] -> [Qayd B] + (Saqlanmagan yangi bitiklar)
2. $ git reset --hard HEAD~1
3. [Qayd A] (Yangi HEAD)
   * [Qayd B] tarihdan öçdi.
   * Savatça (Index) boşadi.
   * Diskdagi yangi bitiklar ham öçirildi.
```

## Etiborga olingan muqobillar

- qattiq qaytaruv
