// 1. 定義結構（集合）
struct Point {
    x: i32,
    y: i32,
}

// 2. 實作區塊 (impl)：把屬於 Point 的函數全部集中寫在這裡
impl Point {
    // 這是一個「關聯函數」，通常用來創造一個新的點（就像代數裡的生成元或建構式）
    // 注意這裡沒有 self 參數
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    // 這是一個「方法（Method）」，注意它的第一個參數是 &self
    // &self 代表「借用自己這個點的資料」。這在數學上相當於 P.is_on_unit_circle()
    fn is_on_unit_circle(&self) -> bool {
        self.x * self.x + self.y * self.y == 1
    }
}

fn main() {
    // 3. 使用我們剛寫好的 new 函數來生成一個點
    // 寫法是 結構名稱::函數名稱
    let test_point = Point::new(0, 1);
    
    // 4. 呼叫方法：直接在點的後面加上「.」來呼叫它專屬的方法
    // 這比 is_on_unit_circle(test_point) 看起來更直覺，也更符合物件導向的思維
    let result = test_point.is_on_unit_circle();
    
    println!("點在單位圓上嗎？ {}", result);
}