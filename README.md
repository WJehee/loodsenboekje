# Loodsen Boekje

Website to keep track of the ways a beer has been opened.

Favicon generated with: https://favicon.io/emoji-favicons/

# Note to self

When updating, the registration passwords change as they are randomly generated on build.
They can be found in the shell script in the nix store path.

# Adding a migration
```
cargo sqlx migrate add <name>
```

# Deploying

Loodsenboekje exports a NixOS module that can be used to deploy it.
Add the flake to your flake inputs and use this line to enable the server:

```
services.loodsenboekje.enable = true;
```

By default, the database and logs are stored in `/var/lib/loodsenboekje`.  
In order for the program to run, a database must be created at that location (using `sqlx`),
or you can copy and existing database (with the correct schema).

## Copying binary from faster machine

If you are running this on a server with little resources (like me) it might be faster to
build the binary locally and then copy it to the server instead of building it on the server.

By default, NixOS trusts the root user to copy store paths.
If you do not access your server with root (as you should), add the following to your server configuration:
```nix
nix.settings.trusted-users = [
    YOUR_USERNAME
];
```

Copying can be done with the just recipe: `just copy TARGET` or by following the steps below

Finding the store path:

- Build the binary using `nix build` then run `nix path-info` to get the path
- The path is also shown when using `systemctl status loodsenboekje` if you install it as a service

Copy the path to the remote server
```
nix copy --to ssh://user@remote /nix/store/PATH_TO_PACKAGE
```

