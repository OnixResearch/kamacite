# Change-local acceptance

a[update-cairn-input.lock] The generated root `cairn` lock node MUST name revision `695124d459574ba7aeba6097310d237f393c243c`, and only Nix MAY generate the lock change.

a[update-cairn-input.migration] The native `.cairn/` tree MUST contain every legacy lifecycle file with the same bytes before the legacy tree is removed.

a[update-cairn-input.paths] Maintained Kamacite policy and operator paths MUST select `.cairn/`; historical external references MAY retain their original spelling.

a[update-cairn-input.validation] Lifecycle validation, policy freshness, focused Kamacite checks, and full flake validation MUST pass after the update.
