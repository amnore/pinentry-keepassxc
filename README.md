# pinentry-keepassxc

A pinentry program that automatically gets gpg passphrase from [KeePassXC](https://github.com/keepassxreboot/keepassxc).

## Setup

1. enable logging (for example by uncommenting line 12 in main.rs)
1. compile this program and add it as an pinentry program [GnuPG Manual / ArchWiki](https://wiki.archlinux.org/title/GnuPG#pinentry)
1. invoke gpg such that a signing request is triggered
1. your keepassxc should ask for a connection name => choose one (here I choose: connectionname)
1. `sh cat ~/.cache/pinentry-keepassxc.log` and look for a message with entry
   example output: ```json {"action":"get-logins","url":"gpg://THIS_IS_A_URL","keys":[{"id":"CONNECTION_NAME","key":"KEY_FIELD_IN_ENTRY_MESSAGE_ABOVE"}]} ```
1. create a config file `nano ~/.config/pinentry-keepassxcrc`
   content: ```json
   {
     "id": "CONNECTION_NAME"
     "idKey": "KEY_FIELD_IN_ENTRY_MESSAGE_ABOVE"
   }
   ```
1. add for the entry in your database add this as an url:  gpg://THIS_IS_A_URL
1. **DISABLE LOGGING AGAIN** - otherwise password will be logged in clear text to the above mentioned file

## Usage


## How it works

See: [docs](https://github.com/MrChenWithCapsule/pinentry-keepassxc/tree/main/docs)

## Other integration utilities for KeePassXC

- [git-credential-keepassxc](https://github.com/Frederick888/git-credential-keepassxc): Git credential helper
- [keepassxc-browser](https://github.com/keepassxreboot/keepassxc-browser): Integrate KeePassXC with web browsers
