# ADR-002 — Original HTML is not trusted UI input

## Decision
Trusted UI receives `SafeDocument`, not raw HTML.

## Rationale
A sanitizer must understand every dangerous interaction in an evolving web platform. The safer
boundary is to reconstruct a smaller typed representation and keep active content out of the UI.

## Future pixel mode
A disposable renderer may later rasterize complex mail and re-encode the result into a normalized
bitmap for a “visual only” mode. This complements, rather than replaces, structured Safe View.
