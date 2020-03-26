use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    if a == 1 {
        return 0;
    }

    let mut a = a;
    let mut g = g;

    let mut result = 1;

    while a > 0 {
        if a % 2 == 1 {
            result = (result * g) % p;
        }
        a >>= 1;
        g = g.pow(2) % p;
    }
    result
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}
