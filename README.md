# PassMNG
PassMNG is a simple TUI Password Manager written in Rust for managing passwords and securing them with help of SQLCipher

Related Youtube Video: [here](https://www.youtube.com/watch?v=7r7HOZReZ60)

## Usage
Just enter the program:
```
passmng
```
Then It will ask for Passphrase:
```
Enter Passphrase:
```
Pay attention to the Passphrase you entered for the first time. It's will be the Passprase of program for the rest.

And that's it.


```
L:           List
U:           On list, It's copy the Username
P:           On list, It's copy the Password
D:           On list, It's Delete
E:           On list, It's Edit
S:           Search
Insert Btn:  Insert new Password
Tab:         Go to next field
Shift+Tab:   Go to previous filed
Esc:         Exit insert mode
```

## Build on Windows
First you need to install OpenSSL using [this](https://wiki.openssl.org/index.php/Binaries) link.

Then you need to add OpenSSL ENV. In PowerShell enter this:
```
setx OPENSSL_DIR "PATH TO OPENSSL FOLDER"
eg: setx OPENSSL_DIR "C:\Program Files\OpenSSL-Win64"
```
then refresh the env:
```
refreshenv
```
Exit the PowerShell and open it again

And finally:
```
cargo build --release
```
Enjoy.