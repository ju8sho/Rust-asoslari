/*i8 = max 255
i16 = 6535
i32 = 456789483
i64 = 23456789234567834562
i128 = mlyard sonlar ga teng
isize = qurulmalarning bit formatiga qarab uzi belgilaydi masalan 64bit razryadlik prosessor
u8 u16 u32 u64 u128 usize  faqat 0 dan yuqori sonlarni qabul qiladi
*/

fn main() {
    let mut son: i64 = 34545545;
    println!("sonlar ={}", son);
    //sonlarni uzgartiramiz
    son = 456734678923;
    println!("sonlar {}", son)
}