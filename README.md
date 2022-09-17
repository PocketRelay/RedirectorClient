# Pocket Relay Redirector

This is a working version of the **Pocket Relay** new system where the redirector server
is hosted on the individual client computers rather than on an external server.

Note this is unfinished at the moment it requires SSLv3 to be enabled through a registry 
tweak and the following ciphers need to be enabled:
- TLS_RSA_WITH_RC4_128_MD5 
- TLS_RSA_WITH_RC4_128_SHA 