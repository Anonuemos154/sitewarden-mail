# Sandbox design

## Windows
Plan separate scanner executables launched with:
- AppContainer / restricted token where practical;
- job object memory/CPU/process limits;
- no child-process creation;
- no network for parser/attachment workers;
- dedicated temp directory;
- low integrity for untrusted artifacts where compatible.

## macOS
- App Sandbox where distribution model allows;
- helper processes with minimal entitlements;
- no outbound network for parser/attachment workers;
- hardened runtime + notarization.

## Linux
- seccomp-bpf profile;
- namespaces;
- no network namespace for parser/attachment workers;
- read-only filesystem view plus ephemeral work directory;
- systemd/bubblewrap-style confinement as packaging permits.

## Generic process protocol
Use a small length-prefixed, versioned binary/JSON protocol. Reject:
- oversized frames;
- unknown protocol versions;
- unexpected message types;
- arbitrary file paths supplied by untrusted content.

Pass file handles/descriptors or controlled temp IDs rather than letting workers browse the user's
filesystem.
