# CI templates

Do not blindly enable release automation until:
- dependency versions/actions are pinned;
- signing secrets are stored in the appropriate secure signing service;
- branch protection is active;
- release provenance process is reviewed.

Security products should not ship from an unreviewed CI template.
