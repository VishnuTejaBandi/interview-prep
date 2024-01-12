#[cfg(test)]
mod tests;

/// vid link https://practice.geeksforgeeks.org/batch/dsa-4/track/DSASP-Mathematics/video/MTg0OQ%3D%3D
pub fn prime_factors(mut n: i32) -> Vec<i32> {
    let mut vec = vec![];

    let mut start = 2;
    while start * start <= n {
        if n % start == 0 {
            n = n / start;
            vec.push(start)
        } else {
            start += 1;
        }
    }

    if n > 1 {
        vec.push(n)
    }

    vec
}

pub fn all_divisors(n: i32) -> Vec<i32> {
    let mut v = vec![];

    fn divisors(n: i32, c: i32, v: &mut Vec<i32>) {
        if n % c == 0 {
            v.push(c);
        }

        if (c + 1) * (c + 1) <= n {
            divisors(n, c + 1, v);
        }

        if n % c == 0 && n / c != c {
            v.push(n / c);
        }
    }

    divisors(n, 1, &mut v);

    v
}

pub fn all_primes_below(n: i32) -> Vec<i32> {
    if n <= 1 {
        return vec![];
    }

    let mut map = vec![1; n as usize - 1];

    let mut current = 2;
    while current <= n {
        if map[current as usize - 2] == 1 {
            let mut multiple = current * current;
            while multiple <= n {
                map[multiple as usize - 2] = 0;

                multiple += current;
            }
        }

        current += 1;
    }

    let mut result = vec![];
    for (index, element) in map.iter().enumerate() {
        if *element == 1 {
            let number = index as i32 + 2;
            result.push(number);
        }
    }

    result
}

pub fn power(n: i32, p: i32) -> i32 {
    if p == 0 {
        return 1;
    }

    let t = power(n, p / 2);
    if p % 2 == 0 {
        return t * t;
    } else {
        return t * t * n;
    }
}

pub fn power_iter(n: i32, mut p: i32) -> i32 {
    let mut res = 1;
    let mut temp = n;
    while p > 0 {
        if p & 1 == 1 {
            res = res * temp;
        }
        p = p >> 1;
        temp = temp * temp;
    }

    res
}

// number of digits in a number = floor(log10(number)) + 1
// log10(5*4*3*2*1) = log10(5) + log10(4)...
pub fn digits_in_factorial(n: usize) -> usize {
    let mut res: f32 = 0.0;

    let mut current: usize = 2;
    while current <= n {
        res = res + (current as f32).log10();
        current += 1;
    }

    res.floor() as usize + 1
}
