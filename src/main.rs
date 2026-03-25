mod field;
mod poly;
// 將目前檔名為 `QAP.rs` 的檔案，以符合 Rust 慣例的模組名 `qap` 引入進 module tree。
#[path = "QAP.rs"]
mod qap;

fn main() {
    // 此專案目前主要用來實作/測試 ZKP 相關數學結構。
    // 若要驗證 QAP/Lagrange 插值流程，可在這裡呼叫對應函數。
    println!("zkp_playground");
}
