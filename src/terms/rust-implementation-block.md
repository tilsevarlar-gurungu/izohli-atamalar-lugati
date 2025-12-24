# Tatbiq Maydoni

**Inglizcha:** Implementation Block<br>
**Ruscha:** Блок реализации<br>
**Aloqali:** Rust<br>
**Soha:** Dasturlov

Rust tilida **tatbiq maydoni** maxsus `impl` (implementation) kalit soʻzi yordamida yaratiladi. Boshqa koʻplab tillardan farqli oʻlaroq, Rustda ma’lumotlar tuzmasi (struct) va uning qobiliyatlari bir-biridan ajratilgan boʻladi: `struct` qismida faqat xususiyatlar e’lon qilinsa, `impl` tatbiq maydonida esa shu tuzmaga tegishli barcha topshiriqlar va mantiqiy bitiklar yoziladi.

Ushbu maydon ichida:

- *Xususiy topshiriqlar:* Faqat shu tuzma ichida ishlaydigan mantiq.
- *Olamshumul boʻlmagan qobiliyatlar:* Jism yaratmasdan turib chaqiriladigan qobiliyatlar (masalan, new() qobiliyati).
- *Xislatlar tatbigʻi:* Biror umumiy xislatni aniq bir jinsga moslab chiqish.

Bu yondashuv bitikning pogʻonalilik tuzumini yaxshilaydi va bitikni qayta ishlashni ancha osonlashtiradi, chunki barcha mantiqiy "harakatlar" bitta jamlangan tatbiq maydoni ichida boʻladi.

## E’tiborga olingan muqobillar

- tatbiq boʻlagi
