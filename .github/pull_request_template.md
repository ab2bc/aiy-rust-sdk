## Anti-injection checklist (mandatory)

- [ ] `gitleaks` ran clean on this branch (`gitleaks detect --source . --no-git -v`)
- [ ] No real secret values added — use `<REDACTED>` placeholders
- [ ] No `.env`, `*.key`, `*.pem`, `*.keystore`, validator keys, or wallet credentials added
- [ ] No new external dependencies introduced without lockfile diff review
- [ ] Workflow files don't print or upload `${{ secrets.* }}` values
- [ ] `CODEOWNERS` / `SECURITY.md` untouched, or reviewed by @ab2bc
- [ ] **Move modules:** no `UpgradeCap`/`TreasuryCap` recipient changes; no visibility widening (`public(friend)` → `public`); no validator-set or threshold changes; no fee-recipient redirects
- [ ] **Config:** no RPC URL substitutions in `.env*`, `deploy-*.mjs`, or `Genesis/`
- [ ] Commit is signed
