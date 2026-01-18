# Qaytaruv

**Inglizça:** Reset<br>
**Rusça:** Сброс<br>
**Aloqali:** Git<br>
**Soha:** Dasturlov<br>

**Qaytaruv** - bu bitikning hozirgi holatini, qayd maydonini yoki tahrirdagi oğoçni loyiha tarihidagi oldingi malum bir nuqtaga (qaydga) qaytariş amalidir. Bilimsanar fanida bu buyruq özgarişlarni bekor qiliş yoki bitik tarihidagi hatolarni tozalaş uçun işlatiladigan eng keskin vositalardan biri hisoblanadi.

Qaytaruv amali uç hil darajada bajarilişi mumkin:

- *Quruq qaytaruv*: Faqat bitik tarihidagi körsatkiç (HEAD) orqaga qaytadi, lekin qilgan özgarişlaringiz qayd maydonida saqlanib qoladi.
- *Yarim qaytaruv*: Bitik tarihi va qayd maydoni orqaga qaytadi, lekin özgarişlar diskda (tahrirdagi oğoçda) saqlanib qoladi.
- *Töla qaytaruv*: Hamma narsa tanlangan nuqtaga qaytadi va barça saqlanmagan özgarişlar butunlay öçirib yuboriladi.

```
[Qayd A] --- [Qayd B] --- [Qayd C] (HEAD)
                             |
         $ git reset --hard [Qayd A]
                             |
[Qayd A] (HEAD)  <-- (B va C butunlay öçirildi)
```

## Etiborga olingan muqobillar

- taşlab yuboriş
