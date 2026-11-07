<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Security policy

NiftyDesign is an unofficial alpha. It opens files you give it, so a malformed file that crashes or hangs the app is a real concern for us, and we want to hear about it.

## What to report as a security problem

- A crafted `.pub`, `.designcraft` or other file that makes the app crash, hang, use huge amounts of memory, or write or read files you did not choose.
- Anything in a release download that does not match its published checksum, or that looks tampered with.
- A secret, password or private file that was published in this repository by mistake.

An ordinary "my file opens with warnings" or "my file looks wrong" report is not a security problem. Use the normal issue forms for those.

## How to report

1. Use GitHub's private reporting: open the **Security** tab of this repository and choose **Report a vulnerability**. This keeps the details private until there is a fix.
2. If that option is not available, open a normal issue titled "Security: please contact me" and do **not** include the file, exploit steps or personal details. We will tell you how to send them safely.

Please include the NiftyDesign version, your Windows version, and what happened. Do not send files containing personal information.

## What to expect

NiftyDesign is maintained by one person with AI help, so we cannot promise a fixed response time. We will acknowledge a report as soon as we can, fix real problems in the next alpha, and say so in the release notes.

## Supported versions

Only the latest alpha release is supported. There are no security updates for older alphas.

## Unsigned downloads

The Windows build is not code-signed yet, so Windows SmartScreen will warn when you run it. Check the SHA-256 value shown on the release page against your download, and only download from this repository's Releases page. We plan to apply for free open-source code signing once the project is public and has a release.
