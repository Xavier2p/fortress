---
title: "Fortress"
sub_title: "DEVSEC Project"
authors:
  - Eliott Delhaye
  - Aurelien Thierry
  - Alexis Galopin
  - Xavier de Place
options:
  end_slide_shorthand: true
theme:
  override:
    table:
      alignment: center
    footer:
      style: template
      left: 2026-01-15
      center: FORTRESS
      right: "{current_slide} / {total_slides}"
      height: 2
---

<!-- jump_to_middle -->

Simple CLI password safe, written in Rust.
---

---

Features
---

- Fully local vault
- Useful in CI operations or with automation tools
- Useful for CLI Lovers
- Full integration with Clipboard to not display the password
- Create as many vault as you want

---

CLI
---

```bash
frtrs create
# ----
frtrs add mail.com -u example@mail.com -p mysecurepassword
# ----
frtrs list
# ----
frtrs view mail.com
# ----
frtrs copy mail.com
# ----
frtrs remove mail.com
```

---

Lifecycle
---

- Agile Methods
- Git and GitHub for collaboration
- CI / CD
- Immutable Releases

---

Good Practices
---

- No storing of master password
- Well known libraries for cryptographic methods
- Strict checks and tests before merging code change
- Weekly audit of all dependencies
- Independent code audit

---

Audit Recommendations
---

| Level    | Description                 | Resolution                 | Implemented |
|----------|-----------------------------|----------------------------|-------------|
| CRITICAL | No Logging System           | Implement a logging system | YES         |
| HIGH     | No Password Policy          | Check password strength    | PARTIALLY   |
| HIGH     | No validation for Clibpoard | Sanitize clipboard content | NO          |

<!-- new_lines: 4 -->

> [!NOTE]
> These vulnerabilities have been found by the group that audited us. We do not fully agree with them for the classification.
---

Organization
---

| Role                | Attributions                       |
|---------------------|------------------------------------|
| Lead Dev            | Architecture / Features and merges |
| Security Consultant | Crypto, libraries and audit        |
| CI Engineer         | Tests, Quality and Pipelines       |
| Product Owner       | Documentation / Releases           |

<!-- pause -->

| Sprint | Description           |
|--------|-----------------------|
| 1      | Architecture / Crypto |
| 2      | Commands for MVP      |
| 3      | Finalize project      |
| 4      | Audit recommendations |

---

Code Audit
---

> [!NOTE]
> Their project is a Client - Server for Zero Knowledge Proofs PoC

<!-- pause -->
<!-- new_lines: 4 -->

| Level    | Description                      | Resolution              |
|----------|----------------------------------|-------------------------|
| CRITICAL | Exposed Providing Key Generation | Delete Endpoint         |
| HIGH     | RNG non crypto safe              | Use other method        |
| HIGH     | Libraries not safe               | Update                  |
| MODERATE | No TLS / dDOS Protection         | Use ratelimit and HTTPS |
| MODERATE | Rust Errors                      | Configure Linting       |

---

<!-- jump_to_middle -->

Thanks for your listening, it's demo time
---