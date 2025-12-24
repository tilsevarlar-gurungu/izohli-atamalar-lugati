# Yongʻoq

**Inglizcha:** Option<br>
**Ruscha:** Опшн<br>
**Soha:** Dasturlov<br>
**Aloqali:** Rust

Rust’da **Yongʻoq** sonlamasi - bu qiymatning mavjud boʻlishi yoki boʻlmasligini aniq ifodalash uchun ishlatiladigan muhim sonlama turidir. U boshqa tillardagi *null* yoki *nil* kabi noaniq qiymatlardan farqli oʻlaroq, dasturchini qiymatning yoʻqligini nazoratli tarzda boshqarishga majburlaydi. Yongʻoq ikki holatdan birini oʻz ichiga oladi: yo qiymat mavjud boʻlsa, Magʻizli Some(T)) holati, yoki qiymat umuman yoʻq boʻlsa, Puch (None) holati. Yongʻoq’dan foydalanish nuqsonlarni (ayniqsa, *null*ga kirish xatolarini) oldini olishga yordam beradi va bitikni oʻqishni yanada ishonchli qiladi.

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

## E’tiborga olingan muqobillar

- tanlov
- opsiya
