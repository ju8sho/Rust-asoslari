/* O'zgaruvchilar
let kalit so'z bilan boshlanadi
let kalit so'zidan keyin o'zgaruvchiga nom beriladi
matinlar va o'zgaruvchilarni ajratish uchun (,) vergul qoyiladi

consolda chiqarish uchun o'zgaruvchilar oldiga qo'sh tirnoq ichida {} katta qavs qoyiladi
bit larni ko'rsatishda: Xotiradan joy korsatish
i8 = max 255
i16 = 6535
i32 = 456789483
i64 = 23456789234567834562
i128 = mlyard sonlar ga teng
isize = qurulmalarning bit formatiga qarab uzi belgilaydi masalan 64bit razryadlik prosessor
u8 u16 u32 u64 u128 usize  faqat 0 dan yuqori sonlarni qabul qiladi
mutable = yozilishi mut o'zgaruvchilarni o'zgartirish
*/
mod amaliyot;

fn main() {
    let mut yosh: i8 = 26;
    yosh = 28;
    yosh = 45;
    println!("{}", yosh);
    //matinlar bilan chiqarish
    println!("Mening yoshim {}", yosh)
}

//// xavsiz, va o'zgaruvchilarni aniq korsatish
// fn main() {
//     let name = String::from("Ju");
//     println!("My name is {}", name)
// }
