
# Recryption Proxy
# Purpose

This technique use homomorphic encryption to allow anyone to selectively reveal and share secret data, but store it anywhere. Files may reference each other, even when stored in different data vaults. Access may be later revoked, or limited to a specific length of time.


# Functional Use Case

Bob wants to access Alice’s secret video file. Alice would like to give Bob temporary access. The file is published in a public data vault, but encrypted. If Alice gives Bob a shared decryption key outright, Bob will have permanent access. How can Alice give Bob access, but later revoke access if she chooses?

The proxy recryption service keeps a list of public keys authorized to decrypt the data. Bob requests the stream with a signed request; his key is confirmed on the access control list, the proxy decrypts the original ciphertext and re-encrypts it so that Bob’s key can decrypt it. The original ciphertext is safe (Only Alice’s key can decrypt it), and access may be rescinded by halting the service or changing the access list.

# Methodology

All calls are authenticated. This authenticated signature is used to derive the pubkey making the call, which is then used to determine if that pubkey has the proper permissions to make the call.

For each call, caller must be authorized to make the call, including for GET requests.

The server can take uploaded encrypted files and save them in an associated data store (S3-compatible object storage API).

Files are encrypted using standard AEAD; the associated data is stored in plaintext alongside the ciphertext, but is included in the authentication signature, so cannot be changed without invalidating the signature. This can include things like MIME type, timestamp, or data schema (ontology). We refer to the encrypted files + metadata that are available to stream from the recryption proxy as “resources”.

The server stores recryption keys for the encrypted resources. This allows a resource to be re-encrypted so that a specific key may decrypt it. If a user key wants access to an encrypted resource, there must be a recryption key available that can recrypt the resource to that key. 

The key that encrypted the file can create a recryption key for that file.

The server also maintains an ACL for each resource describing what permissions users (keypairs) have. If the server holds a recryption key for a user, but that user doesn’t have read permission on the resource, the server will refuse the recryption request.

The available ACL permissions are: read, write, admin, and owner. Read requires a recryption key for the reader. Write is TBD regarding functional requirements. Admin is whether the server allows that user key to alter the permissions for that resource. Owner is an implicit permission, since the owner must be the key that was used for encrypting the resource. Note that only the owner can create recryption keys.

## Hosting Agility and Resilient References

One of the unique quality attributes of this design is that one can maintain dynamic access control on a shared resource, but still have hosting agility for that resource. Which means that the resource can be stored anywhere as long as the storage layer follows the protocol rules for data vaults (which is in general a supserset of S3-compatible object storage). References are resilient: you can use a content-addressed hash and/or IPNS-style pubkey hash to reference the resource, so the storage provider of the document may be changed without invalidating the reference. References across different storage providers are also valid.

## Future

Writing: TBD on the method to be used here, but the intention is for users with the Write Permission to a resource to be able to POST resource bytes, and the proxy should be able to recrypt or encrypt these bytes and write them.

There are some important use cases here of using collaborative append-only data structures like CRDTs to be able to take authenticated, signed updates and keep a validated record of all the updates that have happened while at the same time maintain a single, verifiable, secure, reasonably private encrypted document.

In future iterations, we will add an optional RBAC method for defining resource access, so that roles can be defined and be given access, and then keypairs can be added/removed from the role. Recryption key management will need to be addressed for this use case in more detail.

# API Structure

## Call Authentication
POST Body is JSON with MAC. 

All requests are authenticated and signed by a keypair. Pubkey is extracted from the signature.

Server compares sender’s ACL to determine if it has permission to make the call.

Headers: Authorization: signature (MAC)

- Server extracts signing key out of the signature, compares against ACL
- Timestamp included in the MAC; request is only valid for X seconds
- Include nonce to guard against replay attacks

## REST Calls

### POST /resources
    
    Create new resource. Upload file. POST body is the encrypted file.
    
    Returns the resource descriptor (hash of the content).
    
    MAC of the request, includes timestamp. 
    
    Server extracts pubkey out of MAC, compares against encrypting key of the resource.
    
    Encryption is via AEAD. Additional data can be anything; schema TBD, most likely json blob like JWT.
    
### GET /resources/[hash]
    
    Return the resource, re-encrypted to the key of the sender.
    
### POST /acls
    
    Create new access. This will mostly be created through the POST /recryptions call, if a recryption key is necessary.
    
    Returns an error if there’s no Recryption Key for the accessor, and the permission requested requires one.
    
### GET /acls/[pubkey]
    
    Show the ACLs for the pubkey. 
    
### PUT /acls/[pubkey]
    
    Change the access of a key.
    
### POST /recryptions
    
    Create a new recryption key. Also creates an ACL for that user on that resource.
    
    { resource: [hash], pubkey: [pubkey], recryption_key: [recryption key], permissions: [rwa] }
    
### GET /recryptions
    
    Returns all the recryption keys created by the caller
    
### GET /recryptions/[pubkey|resource]
    
    Returns all the recryption keys created by the pubkey
    
    (Or that pubkey has access to see)
    
### GET /recryptions/for/[pubkey]
    
    Returns all the recryption keys for the pubkey
    
### GET /rights/[pubkey]
    
    Show list of keys and their administrative rights to change access to assets encrypted by the [pubkey].
# Features

- [ ] Runtime configuration changes (Like Sozu) - no restarts
- [ ] Livestream video (Xiu https://github.com/harlanc/xiu)
- [ ] Stream media - resumable
- [ ] Object store should be hosting-agnostic (content addressed)
- [ ] Key-authenticated API to change ACL allowlist
- [ ] Recrypt to the requesting key
- [ ] Read encrypted blob from object store
- [ ] HTTP Request with Pubkey in the header


# Data Schemas

TBD