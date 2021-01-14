# Handling the pinentry side

Gpg-agent talks to a pinentry through stdin/stdout, it sends a series of commands and reads responses from the pinentry. The agent and pinentry speak the [assuan protocol](https://gist.github.com/mdeguzis/05d1f284f931223624834788da045c65).

Pinentry-keepassxc acts as an middle-man between the agent and another pinentry. In addition to that, pinentry-keepassxc handles the following conditions:

1. Pinentry-keepassxc will look up the KeePass database when requested for a passphrase. If a matching entry is found, it will return the passphrase to the agent. Otherwise, it will pass the request to the real pinentry, which then prompts for passphrase.

2. When a passphrase is supplied by the real pinentry, pinentry-keepassxc will save it to the database, so that it can later be supplied to the agent automatically.

To achieve this, pinentry-keepassxc handles the following commands:

- GETPIN  
  This is used by the agent to request a passphrase.
  
- D \<passphrase\>  
  This is the response from the real pinentry when requested for a passphrase.
  
- SETKEYINFO \<keygrip\>  
  This is used by the agent to provide `key-grip`, which can be used to identify the key.