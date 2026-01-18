# Uzilgan boş

**Inglizça:** Detached HEAD<br>
**Rusça:** Отсоединённый HEAD<br>
**Aloqali:** Git<br>
**Soha:** Dasturlov<br>

**Uzilgan boş** - bu tahrirdagi oğoç muayyan bir qölga emas, balki töğridan-töğri aniq bir qaydga işora qilib turgan holatidir. Odatda Git tizimida "boş" (HEAD) biror qölning uçiga yopişgan böladi, ammo siz eskiroq qaydni keltiruv orqali oçganingizda, "boş" qöldan ajralib, yolğiz qayd ustiga tuşadi.

Bu holatda dasturçi bitikni tekşirişi yoki tajribalar ötkazişi mumkin, biroq qilingan yangi qaydlar heç qaysi qölga tegişli bölmaydi. Agar siz uzilgan boş holatida yangi bitik yozsangiz va boşqa qölga ötib ketsangiz, qilgan özgartirişlaringiz "adaşib" qolişi va keyinroq çiqindi yiğuvçi tomonidan öçirib yuborilişi mumkin. Buning oldini oliş uçun uzilgan boşdan yangi qöl oçib olinadi.

```
      (Qöl: ona-qöl)
            v
A --- B --- C (HEAD)  <-- Oddiy holat (Boş qölga boğliq)

$ git checkout B

A --- B (HEAD) --- C  <-- Uzilgan boş (HEAD qöldan ajralgan)
```


## Etiborga olingan muqobillar

- qölsiz uç
