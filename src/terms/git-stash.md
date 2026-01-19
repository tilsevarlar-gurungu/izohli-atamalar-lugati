# Saqlab qöyuv

**Inglizça:** Stash<br>
**Rusça:** Спрятанное<br>
**Aloqali:** Git<br>
**Soha:** Dasturlov<br>

**Saqlab qöyuv** - bu bitikdagi hali yakunlanmagan va qayd qilişga tayyor bölmagan özgarişlarni vaqtinça çetga olib qöyiş amalidir. Bu tüşunça dasturçiga işçi muhitni (tahrirdagi oğoçni) tozalab, boşqa bir şoşilinç vazifaga ötib ketiş imkonini beradi.

Bilimsanar fanida saqlab qöyuv amali göyo "vaqtinça saqlagiç" vazifasini bajaradi. Tasavvur qiling, siz biror murakkab hususiyat ustida işlayapsiz, ammo birdan ona qöldagi muhim hatoni tuzatişingiz kerak bölib qoldi. Siz hali bitmagan işingizni qayd qilişni istamaysiz (çünki u hali çala). Şunda siz saqlab qöyuv buyruğini berasiz, özgarişlar Git hotirasining alohida qismiga "çala qayd" qilinmasdan olib qöyiladi va bitik toza holatga qaytadi. Işni bitirib bölgaç, saqlangan özgarişlarni yana qaytadan bitikka "yopiştiriş" mumkin.

```
[Iş jarayoni]
Bitik: "Alohida hususiyat yozilyapti..." (Saqlanmagan)

$ git stash

Bitik: "Toza holat" (Endi hatoni tuzatiş mumkin)

$ git stash pop

Bitik: "Alohida hususiyat yozilyapti..." (Özgarişlar qaytdi)
```

## Etiborga olingan muqobillar

- saqlov
