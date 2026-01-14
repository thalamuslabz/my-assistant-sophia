# Risk Register

| Risk | Impact | Likelihood | Mitigation | Owner | Status |
|---|---|---|---|---|---|
| Local model availability differs by OS | High | Medium | Define supported model list per OS; fallback strategy | Engineering | Open |
| Secure key storage differences (keychain APIs) | High | Medium | OS-specific adapters with tests | Engineering | Open |
| Packaging/signing complexity | Medium | Medium | Early release pipeline spike | Operations | Open |
| UI parity across shells | Medium | Low | Shared UI component library | Product/Engineering | Open |
| Runtime reliability on startup | High | Medium | Health checks + retry policies | Engineering | Open |
