# Qoplovçi

**Inglizça:** Closure<br>
**Rusça:** Замыкание<br>
**Soha:** Dasturlov

**Qoplovçi** – bu öziga hos topşiriq bölib, u aniqlangan paytda atrofidagi muhitni "qoplab oladi" yoki eslab qoladi, hatto bu muhit mavjud bölişni töhtatgandan keyin ham. Bu şuni anglatadiki, topşiriq özining atrofidagi qamrovdan olingan yerlimas yorliqlarga u taşqarida bajarilayotgan bölsa ham, kiriş va ularni özgartiriş imkoniyatiga ega böladi. Bu kuçli hususiyat qadamlov (currying), keçiktirilgan ijro kabi topşiriqli dasturlov usullarini amalga oşirişga va malumotlar mahfiyligi qoliplarini yaratişga yordam beradi.

### Qoplovçi va nomsiz topşiriq örtasidagi farq

Qoplovçi va nomsiz topşiriq örtasidagi farq nozik, ammo muhimdir. Nomsiz topşiriq (yoki lambda) şunçaki şahsni aniqlovçiga biriktirilmagan (ismi yöq) topşiriqdir. Boşqa tomondan, qoplovçi – bu topşiriqning öziga hos nushasi bölib, u nomsiz bölişi ham mumkin, ammo u özining atrofidagi muhitni qoplab oladi. Şunday qilib, har bir qoplovçi oddiy topşiriqdir, ammo har bir nomsiz topşiriq, agar u atrofidagi doiradan yorliqlarni haqiqatdan ham qoplab olmasa, qoplovçi hisoblanmaydi.

### Rust va Python misolida

#### Python

Pythonda taşqi qavram yorliqqa murojaat qiladigan barça inli topşiriqlar avtomatik ravişda qoplovçini hosil qiladi. Masalan:

```python, ignore
def make_multiplier(x):
    # "x" qoplovchi tomonidan qamrab olingan yerlimas yorliq
    def multiplier(n):
        return x * n
    return multiplier
# "doubler" qoplovchi hisoblanadi; u x=2 ni eslab qoladi
doubler = make_multiplier(2)
print(doubler(5)) # Natija: 10
```

#### Rust

Rustda qoplovçilar `|...|` yozuv qoidalaridan foydalanib aniqlanadigan nomsiz topşiriqlardir. Rust qoplovçisi özining atrofidagi muhitni aniq qoplab oladi va bu egalik qoidalari bilan boğliq.

Quyidagi misolda, `num` yorliqsi qoplovçi tomonidan özgarmas olib turuv sifatida qamrab olinadi:

```rust, ignore
fn main() {
    let num = 5; // Tashqi yorliq
    
    // Qoplovchi "num"ni oʻqish uchun qoplab oladi (immutable borrow)
    let plus_num = |x: i32| x + num; 

    println!("Natija: {}", plus_num(5)); // Natija: 10
    
    // Qoplovchi faqat oʻqish uchun qarz olganligi sababli, biz "num"dan
    // keyinchalik ham foydalana olamiz.
    println!("Dastlabki num: {}", num); // Natija: 5
}
```

Bu aniq işlov beriş qoplovçilarning Rustning egalik va olib turuv qoidalariga qatiy rioya qilişini taminlaydi va hotira havfsizligini kafolatlaydi. Qoplovçini `move` kalit sözi bilan ham aniqlaş mumkin: `let owns_num = move |x: i32| x + num;`, bu holda qoplovçi `num` yorliğining öziga egalik qiladi.


## Etiborga olingan muqobillar

- yopiq
- qamrovli
