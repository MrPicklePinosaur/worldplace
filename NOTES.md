
```js
window.ethereum.networkVersion
```
should give us the chain id of the network they are connected to

```js
window.ethereum.request({ method: 'eth_requestAccounts' })
```
request the user account of metamask

```js
window.ethereum.request({
    method: 'eth_sign',
    params: [address, message],
});
```
ask to sign a message for us, although we get an error that this has been disabled

[list of rpc methods](https://ethereum.org/en/developers/docs/apis/json-rpc/#json-rpc-methods)
