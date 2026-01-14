# Tatbiq Maydoni

**Inglizça:** Implementation Block<br>
**Rusça:** Блок реализации<br>
**Aloqali:** Rust<br>
**Soha:** Dasturlov

Rust tilida **tatbiq maydoni** mahsus `impl` (implementation) kalit sözi yordamida yaratiladi. Boşqa köplab tillardan farqli ölaroq, Rustda malumotlar tuzmasi (struct) va uning amallari bir-biridan ajratilgan böladi: `struct` qismida faqat hususiyatlar elon qilinsa, `impl` tatbiq maydonida esa şu tuzmaga tegişli barça topşiriqlar va mantiqiy bitiklar yoziladi.

Uşbu maydon içida:

- *Hususiy topşiriqlar:* Faqat şu tuzma içida işlaydigan mantiq.
- *Olamşumul bölmagan amallar:* Jism yaratmasdan turib çaqiriladigan amallar (masalan, new() amali).
- *Hislatlar tatbiği:* Biror umumiy hislatni aniq bir jinsga moslab çiqiş.

Bu yondaşuv bitikning poğonalilik tuzumini yahşilaydi va bitikni qayta işlaşni ança osonlaştiradi, çunki barça mantiqiy "harakatlar" bitta jamlangan tatbiq maydoni içida böladi.

## Etiborga olingan muqobillar

- tatbiq bölagi
