/**
 * pubはこのsort関数が他のモジュールからアクセスできることを示します
 * 引数xの型 &mut [u32]について
 *  &は値をポインタ経由で借用することを示す
 *  mutは値がミュータブルであることを示す
 *  u32型は32bit符号なし整数
 *  [u32]型はu32のスライス
 */
pub fn sort<T: Ord>(x: &mut [T], up: bool) {
  if x.len() > 1 {
    let mid_point = x.len() / 2;
    sort(&mut x[..mid_point], true);
    sort(&mut x[mid_point..], false);
    sub_sort(x, up)
  }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool){
  if x.len() > 1 {
    compare_and_swap(x, up);
    let mid_point = x.len() / 2;
    sub_sort(&mut x[..mid_point], up);
    sub_sort(&mut x[mid_point..], up);
  }
}
fn compare_and_swap<T: Ord>(x: &mut [T], up: bool){
  let mid_point = x.len() / 2;
  for i in 0..mid_point {
    if (x[i] > x[mid_point + i]) == up {
      // 要素を交換する
      x.swap(i, mid_point + i)
    }
  }
}

// このモジュールはcargo testを実行したときのみコンパイルされる
#[cfg(test)]
mod tests {
  // 親モジュール(first)のsort関数を使用する
  use super::sort;

  #[test]
  fn sort_u32_ascending() {
    // テストデータとしてu32型のベクタを作成しxに束縛する
    // sort関数によって内容が変更されるので、可変を表すmutキーワードが必要
    let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

    // xのスライスを作成し、sort関数を呼び出す
    // &mut xは & mut x[..]と書いても良い
    sort(&mut x, true);

    // xの要素が昇順にソートされていることを確認する
    assert_eq!(x, vec![4, 10 ,11, 20, 21, 30, 110, 330])
  }
  #[test]
  fn sort_u32_descending() {
    let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
    sort(&mut x, false);
    // xの要素が降順にソートされていることを確認する
    assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10 ,4])
  }
  #[test]
  fn sort_str_ascending() {
    let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
    sort(&mut x, true);
    assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"])

  }
  #[test]
  fn sort_str_descending() {
    let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
    sort(&mut x, false);
    assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"])
  }
}
