# Olib turish

**Inglizcha:** Borrowing<br>
**Ruscha:** Заимствование<br>
**Soha:** Dasturlash


Ishoralar va murojaatlar bilan chambarchas bog'liq bo'lgan tushuncha ko'plab tillarda mavjud, lekin `olib turish`, ya'ni tuzuv vaqtida majburiy tatbiq etiladigan qoidalar tizimi sifatida, faqat Rust tiliga xosdir. Bu tushuncha `egalik` va `umr` bilan bir qatorda tilning xotira xavfsizligi modelining asosiy tarkibiy qismidir.

Olib turish bitikning ma'lumotga egalikni o'ziga olmay, balki murojaat (masalan, `&qiymat` yoki `&mut qiymat`) orqali vaqtinchalik ishlatishga imkon beradi va shu bilan xotira manzilini bir joydan ikkinchi joyga o'tkazish zaruriyatini yo'q qiladi. Rust Olib Turish Tekshiruvchisi (Borrow Checker yoki OTT) quyidagi xavfsizlik qoidasini qat'iy ravishda tatbiq qiladi: bir vaqtning o'zida siz bir yoki bir nechta o'zgarmas o'zgaruvchini olib turishga yoki aniq bir o'zgaruvchan o'zgaruvchini olib turishingiz mumkin, ammo hech qachon ikkalasiga ham bir vaqtda ega bo'la olmaysiz. Bu qat'iy tekshiruv tuzuv vaqtida amalga oshiriladi va shuning uchun C va C++ kabi an'anaviy tillarni qiynaydigan ma'lumotlar poygasi kabi butun bir xatolar sinfini bartaraf etadi. Swift yoki akademik tillar (masalan, Cyclone, Vale) kabi boshqa tillarda o'xshash mexanizmlar mavjud bo'lsa-da, Rust bu turdagi aniq, majburiy olib turishni o'z dizaynining markaziga qo'ygan eng mashhur dasturlash tilidir.

## Aloqador atamalar

- olib turish tekshiruvchisi
- ishora
- egalik
- umr
- tuzuv

## E'tiborga olingan muqobillar

- ijara
- kiralash
- olib turish nazoratchisi
