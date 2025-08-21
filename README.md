# Encrypter
it encrypts numbers to be a secret message ;) that only who got the keys can view
# Future
add feature to share publick key and encode using others public keys 
and add text encrytion :o
# run
it is already compiled and ready to run in console
1. locate encryptic.exe in encryptic > target
2. downloade it (no need for other files as it works with itself [works on my machine that is XD])
3. run using cmd or just double click it :DD
# use
1. generate your keys by presing enter and it should look like this
```
Paste keys (Leave empty to generate new):

private key: 1999
coprime 2: 3540619
public key: 18983977
copy : 1999,3540619,18983977

0 - encode mesage, 1 - decode message
```
2. copy "copy" numbers : 1999,3540619,18983977 and save them for later use
3. enter 0 for encrypting message
4. enter your mesage (0-9 aswell some size limitations [bout 7 numbers but safe for 6 or smaller])
5. get message wich is for me: 7349743
```
0 - encode mesage, 1 - decode message
0
encode message (numbers kek):
12345
12345,1999,18983977
encrypted : 7349743
```
6. then you can decrypt message by entering 1
7. then paste the number and get the message back :DD
```
0 - encode mesage, 1 - decode message
1
decode message (numbers kek):
7349743
decrypted : 12345
0 - encode mesage, 1 - decode message
```
# advanced use
you can give frend x,0,y to give him the ability to encode but not decrypt
```
Paste keys (Leave empty to generate new):
1999,0,18983977
["1999", "0", "18983977"]
private key: 1999
coprime 2: 0
public key: 18983977
copy : 1999,0,18983977

0 - encode mesage, 1 - decode message
0
encode message (numbers kek):
12345
12345,1999,18983977
encrypted : 7349743
0 - encode mesage, 1 - decode message
```
and then decrypt using your full key :DD
# code
code is all rust and can be found in encryptic > src
where main.rs is the main version and main_old and main_new_old are my old versions just for the funny of comments i keep em (u can see me strugling with the compiler kek)