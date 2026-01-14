# Olib turuv

**Inglizça:** Borrowing<br>
**Rusça:** Заимствование<br>
**Soha:** Dasturlov

Rust tilida **olib turuv** bu egalik tizimining bir qismi bölib, yorliqlarga ularning qiymatidan vaqtinçalik foydalaniş imkoniyatini beriş uçun işlatiladi. Bu jarayonda dasturçi qaratqiç yaratadi, bu aslida qaralmişga işora qiluvçi manzil körsatkiçi ustidagi havfsiz mavhumlaştirmadir. Bu qaratqiç yordamida qaralmişga ötiş mumkin. Tuzgiç tarkibidagi mahsus vosita - olib turuv tekşiruvçisi, tuzuv vaqtida qoidalar töplamiga rioya qilinişini taminlaydi. Eng muhimi, bir vaqtning özida bitta özgaruvçan qaratqiç va çeklanmagan sondagi özgarmas qaratqiçlar mavjud bölişi mumkin. Bu qoida manzil körsatkiçlari orqali bevosita hotiraga kirişdan kelib çiqadigan köplab hotira bilan aloqali hatolarning oldini oladi.

### Olib turuvning bitikda işlaşi

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


## Etiborga olingan muqobillar

- ijara
- kiralaş
- olib turuv nazoratçisi
