# Olib turuv

**Inglizcha:** Borrowing<br>
**Ruscha:** Заимствование<br>
**Soha:** Dasturlov

Rust tilida **olib turuv** bu egalik tizimining bir qismi boʻlib, yorliqlarga ularning qiymatidan vaqtinchalik foydalanish imkoniyatini berish uchun ishlatiladi. Bu jarayonda dasturchi qaratqich yaratadi, bu aslida qaralmishga ishora qiluvchi manzil ko‘rsatkichi ustidagi xavfsiz mavhumlashtirmadir. Bu qaratqich yordamida qaralmishga o‘tish mumkin. Tuzgich tarkibidagi maxsus vosita - olib turuv tekshiruvchisi, tuzuv vaqtida qoidalar toʻplamiga rioya qilinishini ta’minlaydi. Eng muhimi, bir vaqtning oʻzida bitta oʻzgaruvchan qaratqich va cheklanmagan sondagi oʻzgarmas qaratqichlar mavjud boʻlishi mumkin. Bu qoida manzil ko‘rsatkichlari orqali bevosita xotiraga kirishdan kelib chiqadigan koʻplab xotira bilan aloqali xatolarning oldini oladi.

### Olib turuvning bitikda ishlashi

```rust, ignore
fn main() {
    let mut x = 10;           // x qiymatga egalik qiladi

    let r1 = &x;              // oʻzgarmas qaratqich
    let r2 = &x;              // yana bir oʻzgarmas qaratqich
    println!("{}, {}", r1, r2);

    // r1 va r2 bu nuqtadan keyin ishlatilmaydi

    let r3 = &mut x;          // oʻzgaruvchan qaratqich
    *r3 += 5;                 // manzildan qaralmishga o‘tish va qiymatni oʻzgartirish

    // Quyidagi satrdagi bitik tuzgich xatosiga sabab boʻladi:
    // let y = *r1;

    // Yuqoridagi satrda r1’ni ishlatib boʻlmaydi!
    // Xato sababi: Oʻzgarmas qaratqich (r1) hali ham ishlatilayotgan bir paytda,
    // oʻzgaruvchan qaratqich (r3) yaratildi va u orqali (x) oʻzgartirildi.
    // Rust bir vaqtning oʻzida x’ga oʻzgarmas qaratqich (oʻqish)
    // va oʻzgaruvchan qaratqich (yozish) berishga ruxsat bermaydi.
    // Bu ma’lumot poygasini oldini olish uchun
    // Olib Turuv Tekshiruvchisi tomonidan qoʻyilgan qoida.

    println!("{}", x);        // x yana ishlatishga yaroqli
}
```


## E’tiborga olingan muqobillar

- ijara
- kiralash
- olib turuv nazoratchisi
