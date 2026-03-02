// 1. 定義我們專屬的數學結構
// 這就像在宣告：令 Point 為一個包含整數 x 與整數 y 的空間元素
struct Point {
    x: i32,
    y: i32,
}

// 2. 改寫函數：現在定義域的型別不是 (i32, i32) 了，而是我們自己發明的 Point！
fn is_on_unit_circle(p: Point) -> bool {
    // 在 struct 中，我們使用「點 (.)」來取得裡面的特定維度數值
    p.x * p.x + p.y * p.y == 1
}

fn main() {
    // 3. 實例化 (Instantiation)：創造一個具體的點
    // 寫法比 Tuple 明確很多，直接標明 x 是 0，y 是 1
    let test_point = Point { x: 0, y: 1 };
    
    // 4. 把這個自訂結構丟進函數裡
    let result = is_on_unit_circle(test_point);
    
    println!("點在單位圓上嗎？ {}", result);
}