otp
===

Just because I wanted a simple cli tool to generate totp codes.

# deps

Written in [Rust](https://www.rust-lang.org).

# building

Start trying to add an MFA device somewhere. Keep a safe record of the secret you're given and any backup codes (password managers like Lastpass/Keepass/1Password are great for this). Then run the following:

```
SECRET=hunter2 cargo build --release
```

This will produce the executable `target/release/otp`. Use it however you want.

Put it on your path, rename it, compile a separate one for separate services (don't share the same secret between services). I make no guarantees on the usefullness nor security of this code.
