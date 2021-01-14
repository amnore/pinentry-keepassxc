# Communication between pinentry-keepassxc and KeePassXC

Pinentry-keepassxc uses KeePassXC's browser integration interface to communicate with it. Description of the protocol can be found at [keepassxc-protocol](https://github.com/keepassxreboot/keepassxc-browser/blob/develop/keepassxc-protocol.md)

In addition, when an action has failed, KeePassXC will reply with a error message. The format is as follows:

```json
{
	"action": "<the requested action>",
	"errorCode": "<an error code>",
	"error": "<error message>"
}
```

Defination of error codes and messages can be found at KeePassXC's [BrowserAction.cpp](https://github.com/keepassxreboot/keepassxc/blob/develop/src/browser/BrowserAction.cpp).

The possible values of error code are:

- 1 (ERROR\_KEEPASS\_DATABASE\_NOT\_OPENED)
- 2 (ERROR\_KEEPASS\_DATABASE\_HASH\_NOT\_RECEIVED)
- 3 (ERROR\_KEEPASS\_CLIENT\_PUBLIC\_KEY\_NOT\_RECEIVED)
- 4 (ERROR\_KEEPASS\_CANNOT\_DECRYPT\_MESSAGE)
- 5 (ERROR\_KEEPASS\_TIMEOUT\_OR\_NOT\_CONNECTED)
- 6 (ERROR\_KEEPASS\_ACTION\_CANCELLED\_OR\_DENIED)
- 7 (ERROR\_KEEPASS\_CANNOT\_ENCRYPT\_MESSAGE)
- 8 (ERROR\_KEEPASS\_ASSOCIATION\_FAILED)
- 9 (ERROR\_KEEPASS\_KEY\_CHANGE\_FAILED)
- 10 (ERROR\_KEEPASS\_ENCRYPTION\_KEY\_UNRECOGNIZED)
- 11 (ERROR\_KEEPASS\_NO\_SAVED\_DATABASES\_FOUND)
- 12 (ERROR\_KEEPASS\_INCORRECT\_ACTION)
- 13 (ERROR\_KEEPASS\_EMPTY\_MESSAGE\_RECEIVED)
- 14 (ERROR\_KEEPASS\_NO\_URL\_PROVIDED)
- 15 (ERROR\_KEEPASS\_NO\_LOGINS\_FOUND)
- 16 (ERROR\_KEEPASS\_NO\_GROUPS\_FOUND)
- 17 (ERROR\_KEEPASS\_CANNOT\_CREATE\_NEW\_GROUP)
