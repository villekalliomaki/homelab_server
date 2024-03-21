# Disk encryption

Full disk encryption is enabled with arch_install. This means that on a headless server there has to be some way to input the password when rebooting the system. Some options are:

-   Yubikey static password
-   Wireless/wired keyboard without a display
-   Wireless/wired keyboard with input switching monitor

# Backup encryption

All remote and local backups are encrypted with restic. All backup repositories should have different passwords and stored on the host owned and only readable by root. The passwords are on the encrypted filesystem.
