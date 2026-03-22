mod field;
mod curve;

use field::FieldElement;
use curve::{Curve, ECPoint};

fn main() {
    let p = 17;

    // 1. 建立玩具版 secp256k1 曲線 y^2 = x^3 + 7 (mod 17)
    let a = FieldElement::new(0, p);
    let b = FieldElement::new(7, p);
    let my_curve = Curve { a, b };

    println!("=== 橢圓曲線公私鑰生成系統 ===");

    // 2. 定義基底點 G (Generator Point)
    // 我們選 (1, 5) 作為全網公認的起點
    let x_g = FieldElement::new(1, p);
    let y_g = FieldElement::new(5, p);
    let g = ECPoint::Point { x: x_g, y: y_g };
    
    print!("全網公認基底點 G: ");
    g.print();
    println!("---------------------------------");

    // 3. 創建 Alice 的錢包！
    // Alice 的私鑰 (Private Key) 是一個只有她知道的隨機數字
    let alice_private_key: u64 = 10; 
    println!("🔑 Alice 的私鑰 (k): {}", alice_private_key);

    // 4. 計算 Alice 的公鑰 (Public Key) = k * G
    let alice_public_key = my_curve.scalar_mul(alice_private_key, g);
    
    print!("🌍 Alice 的公鑰 (P): ");
    alice_public_key.print();

    // 驗證公鑰是否還在曲線上 (這是一定要的！)
    println!("公鑰是否合法 (在曲線上)？ {}", my_curve.contains(alice_public_key));
    println!("---------------------------------");
    
    // 5. 創建 Bob 的錢包來對比
    let bob_private_key: u64 = 3;
    println!("🔑 Bob 的私鑰 (k): {}", bob_private_key);
    let bob_public_key = my_curve.scalar_mul(bob_private_key, g);
    print!("🌍 Bob 的公鑰 (P): ");
    bob_public_key.print();
}