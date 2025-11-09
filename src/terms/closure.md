# Qoplovchi

**Inglizcha:** Closure<br>
**Ruscha:** Замыкание<br>
**Soha:** Dasturlash

**Qoplovchi** – bu oʻziga xos topshiriq boʻlib, u aniqlangan paytda atrofidagi muhitni "qoplab oladi" yoki eslab qoladi, hatto bu muhit mavjud boʻlishni toʻxtatgandan keyin ham. Bu shuni anglatadiki, topshiriq oʻzining atrofidagi doiradan olingan yerli boʻlmagan oʻzgaruvchilarga u tashqarida bajarilayotgan boʻlsa ham, kirish va ularni oʻzgartirish imkoniyatiga ega boʻladi. Bu kuchli xususiyat qadamlash (currying), kechiktirilgan ijro kabi topshiriqli dasturlov usullarini amalga oshirishga va maʼlumotlar maxfiyligi qoliplarini yaratishga yordam beradi.

### Qoplovchi va nomsiz topshiriq oʻrtasidagi farq

Qoplovchi va nomsiz topshiriq oʻrtasidagi farq nozik, ammo muhimdir. Nomsiz topshiriq (yoki lambda) shunchaki shaxsni aniqlovchiga biriktirilmagan (ismi yoʻq) topshiriqdir. Boshqa tomondan, qoplovchi – bu topshiriqning oʻziga xos nusxasi boʻlib, u nomsiz boʻlishi ham mumkin, ammo u oʻzining atrofidagi muhitni qoplab oladi. Shunday qilib, har bir qoplovchi oddiy topshiriqdir, ammo har bir nomsiz topshiriq, agar u atrofidagi doiradan oʻzgaruvchilarni haqiqatdan ham qoplab olmasa, qoplovchi hisoblanmaydi.

## Rust va Python misolida

### Python

Python'da tashqi doiradagi oʻzgaruvchiga murojaat qiladigan barcha inli topshiriqlar avtomatik ravishda qoplovchini hosil qiladi. Masalan:

```python, ignore
def make_multiplier(x):
    # 'x' qoplovchi tomonidan qamrab olingan yerli boʻlmagan oʻzgaruvchi
    def multiplier(n):
        return x * n
    return multiplier
# 'doubler' qoplovchi hisoblanadi; u x=2 ni eslab qoladi
doubler = make_multiplier(2)
print(doubler(5)) # Natija: 10
```

### Rust

Rustda qoplovchilar `|...|` yozuv qoidalaridan foydalanib aniqlanadigan nomsiz topshiriqlardir. Rust qoplovchisi oʻzining atrofidagi muhitni aniq qoplab oladi va bu egalik qoidalari bilan bogʻliq.

Quyidagi misolda, `num` oʻzgaruvchisi qoplovchi tomonidan oʻzgarmas olib turish sifatida qamrab olinadi:

```rust, ignore
fn main() {
    let num = 5; // Tashqi oʻzgaruvchi
    
    // Qoplovchi 'num'ni oʻqish uchun qoplab oladi (immutable borrow)
    let plus_num = |x: i32| x + num; 

    println!("Natija: {}", plus_num(5)); // Natija: 10
    
    // Qoplovchi faqat oʻqish uchun qarz olganligi sababli, biz 'num'dan 
    // keyinchalik ham foydalana olamiz.
    println!("Dastlabki num: {}", num); // Natija: 5
}
```

Bu aniq ishlov berish qoplovchilarning Rustning egalik va olib turish qoidalariga qatʼiy rioya qilishini taʼminlaydi va xotira xavfsizligini kafolatlaydi. Qoplovchini `move` kalit soʻzi bilan ham aniqlash mumkin: `let owns_num = move |x: i32| x + num;`, bu holda qoplovchi `num` oʻzgaruvchisini oʻziga egalik qiladi.


## Aloqador atamalar

- topshiriq
- nomsiz topshiriq
- topshiriqli dasturlov
- qadamlash (karrilash, currying)
- shaxsni aniqlovchi (identifikator)

## E'tiborga olingan muqobillar

- yopiq
- qamrovli
