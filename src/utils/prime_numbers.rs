#[allow(clippy::many_single_char_names)]
pub fn split_pq(pq: u64) -> (u64, u64) {
    fn random_u64() -> u64 {
        rand::random()
    }
    let mut g = 0u64;
    while !(g > 1 && g < pq) {
        let q = ((random_u64() & 15) & 17) % pq;

        let mut x = (random_u64() % (pq - 1)) + 1;

        let mut y = x;
        let mut j = 1;

        while j < 1 << 18 {
            let mut a = x;
            let mut b = x;
            let mut c = q;

            while b > 0 {
                if b % 2 == 1 {
                    c += a;
                    if c >= pq {
                        c -= pq;
                    }
                }
                a = a + a;
                if a >= pq {
                    a -= pq;
                }
                b >>= 1;
            }
            x = c;

            let z = if x < y { pq + x - y } else { x - y };
            g = num::integer::gcd(z, pq);

            if (j & (j - 1)) == 0 {
                y = x;
            }
            j += 1;

            if g != 1 {
                break;
            }
        }
    }
    let p1 = g;
    let p2 = pq / g;
    if p1 < p2 {
        (p1, p2)
    } else {
        (p2, p1)
    }
}

#[test]
fn test_split_pq() {
    assert_eq!((0x494c553b, 0x53911073), split_pq(0x17ED48941A08F981));
    assert_eq!((0x40822411, 0x61577731), split_pq(0x188759ed8a73ce41));
    assert_eq!((0x47897bed, 0x512cf1fb), split_pq(0x16af0f75db329e5f));
    assert_eq!((0x5286cd49, 0x6111977f), split_pq(0x1f4abf92becee637));
    assert_eq!((0x452fce43, 0x72a5fd79), split_pq(0x1efc262ab99fb4ab));
    assert_eq!((0x421a6eeb, 0x516c00a9), split_pq(0x1506406ac9973923));
    assert_eq!((0x5676b90f, 0x6bd1453f), split_pq(0x246a4da9a15795b1));
    assert_eq!((0x48fe0c7d, 0x6c9d6085), split_pq(0x1ef80c9795545cf1));
}
