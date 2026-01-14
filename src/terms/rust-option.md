# Yonğoq

**Inglizça:** Option<br>
**Rusça:** Опшн<br>
**Soha:** Dasturlov<br>
**Aloqali:** Rust

Rustda **Yonğoq** sonlamasi - bu qiymatning mavjud bölişi yoki bölmasligini aniq ifodalaş uçun işlatiladigan muhim sonlama turidir. U boşqa tillardagi *null* yoki *nil* kabi noaniq qiymatlardan farqli ölaroq, dasturçini qiymatning yöqligini nazoratli tarzda boşqarişga majburlaydi. Yonğoq ikki holatdan birini öz içiga oladi: yo qiymat mavjud bölsa, Mağizli Some(T)) holati, yoki qiymat umuman yöq bölsa, Puç (None) holati. Yonğoqdan foydalaniş nuqsonlarni (ayniqsa, *null*ga kiriş hatolarini) oldini olişga yordam beradi va bitikni öqişni yanada işonçli qiladi.

```rust,ignore
// Yongʻoq (Option) turini e’lon qilish
enum Yongʻoq<J: jins> { // J: jins (T: type)
    Puch,            // Qiymat yoʻq (None)
    Magʻizli(J),     // J jinsidagi qiymat mavjud (Some(J))
}

// i32 jinsidagi Yongʻoq qaytaruvchi topshiriq (function)
fn natija_olish() -> Yongʻoq<i32> {
    // Puch qiymat qaytaramiz (None)
    Yongʻoq::Puch
}

let natija = natija_olish();

// Yongʻoq turini qolipga solish
match natija {
    Yongʻoq::Magʻizli(son) => {
        println!("Magʻiz topildi: {}", son);
    }
    Yongʻoq::Puch => {
        println!("Puch, hech qanday natija yoʻq.");
    }
}
```

## Etiborga olingan muqobillar

- tanlov
- opsiya
