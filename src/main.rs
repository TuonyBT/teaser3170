//  Each of George and Martha must make one initial climb halfway up the stairs, followed by different whole numbers
//  of half-up/half-down pairs. Let these numbers be g and m respectively

//  At a constant 7 steps per second, Martha's total time until she overtakes George halfway is:
//  (1/2 + m * 1/2) * 168 / 7 (up) + (m * 1/2) * 168 / 7 (down), which simplifies to: 12 * (2 * m + 1)

//  At a constant N steps per second up and N+1 down, George's total time until Martha overtakes him halfway is:
//  (1/2 + g * 1/2) * 168 / N (up) + (g * 1/2) * 168 / (N+1) (down), which simplifies to: 12 * 7 * ((g + 1)/N + g/(N + 1))

//  At the time they meet, these times are equal. Therefore we need to find values for m, g and N which equate the two
//  i.e. 2 * m + 1 = 7 * ((g + 1)/N + g/(N + 1))
//  Since both sides of the equation must be multples of 7, 2 * m + 1 = k.7, so m = (7 * k - 1) / 2
//  Also, we know that the total time is less than 10 minutes, so 12 * (2 * m + 1) < 600, i.e. 84k < 600, so k < 8
//  Further, for m to be a whole number, (7 * k - 1) must be even, so k must be odd
//  This defines the first loop for k, from which we derive m
//  The puzzle implies that both N and N + 1 are less than 7, so this defines the loop for N
//  Rearranging the equation for Martha and George's times, we can derive g as a ratio between a numerator and a denominator
//  This ratio must be a whole number, and m even <=> g even, which constraints give a unique solution
fn main() {

    for k in (1..8).step_by(2) {
        let m = (7 * k - 1) / 2;
        for n in 1..6 {
            let numerator = k * n * (n + 1) - (n + 1);
            let denominator = 2 * n + 1;
            if numerator % denominator == 0 {
                let g = numerator / denominator;
                if g % 2 == m % 2 {
                    let t = (2 * m + 1) * 12;
                    println!("n: {}, m: {}, g: {}, m%2: {}, g%2: {}", n, m, g, m % 2, g % 2);
                    println!("Martha overtakes George halfway after {} seconds.", t);
                }
            }
        }
}

    }



