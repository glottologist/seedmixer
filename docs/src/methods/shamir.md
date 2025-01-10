# Shamir Sharing

Shamir's Secret Sharing is a cryptographic algorithm developed by Adi Shamir in 1979. It is used to divide a secret (e.g., a cryptographic key) into multiple shares, such that a subset of these shares can reconstruct the original secret, but fewer than the required subset reveals nothing about the secret.

## Key Concepts

### Secret Sharing

- A technique to distribute a secret among a group of participants, each holding a share of the secret.
- Any predefined subset of shares can reconstruct the secret.

### Threshold Scheme

- **Threshold (t):** The minimum number of shares needed to reconstruct the secret.
- **Total Shares (n):** The total number of shares distributed.

The scheme is denoted as **(t, n)**, meaning that any `t` or more shares can reconstruct the secret, but fewer than `t` shares cannot.

Shamir sharing works by constructing a random polynomial of degree t-1 where the secret is hidden as a constant (which is why it needs a numerical representation). The polynomial is sampled n times to create the shares. t of those shares are then enough to reconstruct the polynomial and recover the secret.

### Lagrange Interpolation

Lagrange interpolation is the technique of reconstructing a polynomial of rank `n-1` given a set of `n` distinct points sampled from the polynomial.

The basis of the technique is to find `n` polynomials where each one passed through one and one point only of the sampled data and is zero at all other points, this point is also given a coefficent which is the recipricol of the polynomial evaluated at the x point.

To put this more formally:

$$P(x)=\Sigma^{n}_{j=1} P_j (x)$$

where:

$$P(x)=\prod^{n}_{k=1,k \neq j} \frac{x - x_k}{x_j - x_k} $$

### Simplified Example

Let's assume that the polynomial we want to recover is:

$$
p(x)=-x^2+4x-1
$$

and the samples we have are:

| x   | y   |
| --- | --- |
| 1   | 2   |
| 3   | 2   |
| 4   | -1  |

The Polynomial will be of the dollowing form:

$$
P(x)=l_1(x)+l_2(x)+l_3(x)
$$

where $$l_n(x)$$ are the Lagrange polynomials.

If we take the first point in the sample `(1,2)`. We first set the other points to be 0 at their x point, i.e. we make them roots.

$$
l_1(x)=(x-3)(x-4)
$$

We then evaluate this for the x coordinate at the first point `(1,2)` .

$$
l_1(1)=6
$$

We then scale by `y` value of the first point as per the equation. The full Lagrange polynomial is then:

$$
l_1(x)=y_1\frac{1}{6}(x-3)(x-4)=\frac{2}{6}(x-3)(x-4)=\frac{1}{3}(x-3)(x-4)
$$

We repeat the process for the other points:

$$
l_2(x)=-(x-1)(x-4)
$$

$$
l_3(x)=-\frac{1}{3}(x-1)(x-3)
$$

The original polynomial can be reconstructed by summing the lagrange polynomials:

$$
P(x)=l_1(x)+l_2(x)+l_3(x)=\frac{1}{3}(x-3)(x-4)-(x-1)(x-4)-\frac{1}{3}(x-1)(x-3)
$$

$$
\therefore P(x)=4x-x^2+1
$$

## How seedmixer uses Shamir sharing

The index of each word is used as a secret for a single Shamir share. The Shamir process is repeated for all word indices for the seed phrase. The shares for all indices are then collated together and stored in a single share file.
