# TOTP (Time based One Time Password)

Core idea of the TOTP based on the RFC 6238 is

$TOTP=HTOP(k, floor(\frac{current-time}{30}))$\
Where,
- `k` = secret key
- 30 seconds = time step
- HTOP = HMAC-based OTP
