# TOTP (Time based One Time Password)

What is OTP(One-time Password)?\
OTP 

Core idea of the TOTP based on the RFC 6238 is

$TOTP=HOTP(k, floor(\frac{current-time}{30}))$\
Where,
- `k` = secret key
- 30 seconds = time step
- HOTP = HMAC-based OTP

-----
What is HOTP?\
HOTP

HMAC -> Hash-based Message Authentication Code