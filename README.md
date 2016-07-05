otp
===

Just because I wanted a simple cli tool to generate totp codes.

# deps

Written in [Go](golang.org), uses [gb](https://getgb.io) as the build tool. TL;DR (with Go installed)-

```
make gb
```

# building

Start trying to add an MFA device somewhere. Keep a safe record of the secret you're given and any backup codes (password managers like Lastpass/Keepass/1Password are great for this). Then run the following (where `$SECRET` is the secret)

```
make SECRET=$SECRET
```

This will produce `bin/otp`. Use it however you want.

Put it on your path, rename it, compile a separate one for separate services (don't share the same secret between services). I make no guarantees on the usefullness nor security of this code.

