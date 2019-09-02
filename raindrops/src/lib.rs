const PLING: &str = "Pling";
const PLANG: &str = "Plang";
const PLONG: &str = "Plong";

pub fn raindrops(n: u32) -> String {
    let is_pling = n % 3 == 0;
    let is_plang = n % 5 == 0;
    let is_plong = n % 7 == 0;

    let mut result = String::new();
    if is_pling {
        result.push_str(PLING);
    }
    if is_plang {
        result.push_str(PLANG);
    }
    if is_plong {
        result.push_str(PLONG);
    }
    if result.is_empty() {
        result = n.to_string();
    }
    result
}
