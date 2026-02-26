// 這個函數接收一個座標點，回傳 true 或 false
fn is_on_unit_circle(point: (i32, i32)) -> bool {
    let (x, y) = point;
    // 判斷 x平方 + y平方 是否等於 1
    x * x + y * y == 1
}

fn main() {
    let test_point = (0, 1);
    let result = is_on_unit_circle(test_point);
    
    println!("點 (0, 1) 在單位圓上嗎？ {}", result);
}