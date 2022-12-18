/// Iter is an iterator over constant arrays.
pub struct Iter<const N: usize, T> {
    pub v: [T; N],
    pub n: usize,
}

impl<const N: usize, T: Copy> Iterator for Iter<N, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.n == N {
            return None;
        }
        let item = self.v[self.n];
        self.n += 1;
        Some(item)
    }
}

pub fn uint_from_bytes<T>(s: &[u8]) -> T
where
    T: From<u8> + std::ops::MulAssign + std::ops::AddAssign,
{
    let (x, len) = uint_from_bytes_prefix(s);
    assert_ne!(len, 0);
    x
}

pub fn int_from_bytes<T>(s: &[u8]) -> T
where
    T: From<i8> + std::ops::MulAssign + std::ops::AddAssign,
{
    let (x, len) = int_from_bytes_prefix(s);
    assert_ne!(len, 0);
    x
}

pub fn uint_from_bytes_prefix<T>(s: &[u8]) -> (T, usize)
where
    T: From<u8> + std::ops::MulAssign + std::ops::AddAssign,
{
    let mut n = T::from(0);
    for (i, &c) in s.iter().enumerate() {
        let r = match c {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            _ => return (n, i),
        };
        n *= T::from(10);
        n += T::from(r);
    }
    (n, s.len())
}

pub fn int_from_bytes_prefix<T>(s: &[u8]) -> (T, usize)
where
    T: From<i8> + std::ops::MulAssign + std::ops::AddAssign,
{
    let mut n = T::from(0);
    let mut signum = 1;
    for (i, &c) in s.iter().enumerate() {
        let r = match c {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            b'-' => {
                debug_assert_eq!(i, 0);
                signum = -1;
                continue;
            }
            _ => {
                n *= T::from(signum);
                return (n, i);
            }
        };
        n *= T::from(10);
        n += T::from(r);
    }

    n *= T::from(signum);
    (n, s.len())
}
