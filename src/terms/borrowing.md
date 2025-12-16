# Olib turuv

**Inglizcha:** Borrowing<br>
**Ruscha:** Заимствование<br>
**Soha:** Dasturlash

Rust tilida <abbr title="borrowing">olib turuv</abbr> bu <abbr title="ownership">egalik</abbr> tizimining bir qismi boʻlib, <abbr title="variable">yorliq</abbr>larga ularning qiymatidan vaqtinchalik foydalanish imkoniyatini berish uchun ishlatiladi. Bu jarayonda dasturchi <abbr title="reference">qaratqich</abbr> yaratadi, bu aslida <abbr title="referenced object">qaralmish</abbr>ga ishora qiluvchi <abbr title="pointer">manzil ko‘rsatkichi</abbr> ustidagi xavfsiz <abbr title="abstraction">mavhumlashtirma</abbr>dir. Bu <abbr title="reference">qaratqich</abbr> yordamida <abbr title="deref reference">qaralmishga o‘tish</abbr> mumkin. <abbr title="compiler">Tuzgich</abbr> tarkibidagi maxsus vosita - <abbr title="borrow checker">olib turuv tekshiruvchisi</abbr>, <abbr title="compilation">tuzuv</abbr> vaqtida qoidalar toʻplamiga rioya qilinishini ta'minlaydi. Eng muhimi, bir vaqtning oʻzida bitta <abbr title="mutable">oʻzgaruvchan</abbr> qaratqich va cheklanmagan sondagi <abbr title="immutable">oʻzgarmas</abbr> qaratqichlar mavjud boʻlishi mumkin. Bu qoida manzil ko‘rsatkichlari orqali bevosita xotiraga kirishdan kelib chiqadigan koʻplab xotira bilan aloqali xatolarning oldini oladi.

## Olib turuvning bitikda ishlashi

```rust, ignore
fn main() {
    let mut x = 10;           // x qiymatga egalik qiladi

    let r1 = &x;              // oʻzgarmas qaratqich
    let r2 = &x;              // yana bir oʻzgarmas qaratqich
    println!("{}, {}", r1, r2);

    // r1 va r2 bu nuqtadan keyin ishlatilmaydi

    let r3 = &mut x;          // oʻzgartiruvchi qaratqich
    *r3 += 5;                 // manzildan qaralmishga o‘tish va qiymatni oʻzgartirish

    // Quyidagi satrdagi bitik tuzgich xatosiga sabab boʻladi:
    // let y = *r1;

    // Yuqoridagi satrda r1'ni ishlatib boʻlmaydi!
    // Xato sababi: Oʻzgarmas qaratqich (r1) hali ham ishlatilayotgan bir paytda,
    // oʻzgaruvchan qaratqich (r3) yaratildi va u orqali (x) oʻzgartirildi.
    // Rust bir vaqtning oʻzida x'ga oʻzgarmas qaratqich (oʻqish)
    // va oʻzgaruvchan qaratqich (yozish) berishga ruxsat bermaydi.
    // Bu ma'lumot poygasini oldini olish uchun
    // Olib Turuv Tekshiruvchisi tomonidan qoʻyilgan qoida.

    println!("{}", x);        // x yana ishlatishga yaroqli
}
```

## Aloqali atamalar

- olib turuv tekshiruvchisi (borrow checker)
- qaratqich (reference)
- qaralmish (referenced)
- qaralmishga oʻtish (dereference)
- manzil koʻrsatkichi (pointer)
- manzildan qaralmishga oʻtish (dereference pointer)
- egalik (ownership)
- tuzgich (compiler)
- tuzuv (compilation)
- ma'lumot poygasi (data race)

## E'tiborga olingan muqobillar

- ijara
- kiralash
- olib turuv nazoratchisi
