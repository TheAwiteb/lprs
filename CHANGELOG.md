# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## unreleased
### Added
-  Ability to edit & remove by name (not index only) ([`d8350e6`](https://git.4rs.nl/awiteb/lprs/commit/d8350e636e733c6d49d46d95e0c3ca6c403d72c5))
-  Ability to enter password via stdin `add`&`edit` ([**#15**](https://git.4rs.nl/awiteb/lprs/issues/15)) ([`5f357b8`](https://git.4rs.nl/awiteb/lprs/commit/5f357b89cb6a49be1c5461fa4b6cd5aaec8e541f))
-  Ability to pass the master password as option ([`62d4060`](https://git.4rs.nl/awiteb/lprs/commit/62d4060bb8ffdfb834d5c860f79414cbca211f72))
-  Add `--json` flag to the `list` command ([`a7ad8b4`](https://git.4rs.nl/awiteb/lprs/commit/a7ad8b468277aa5bc1df8616d93b757c3eab303f))
-  Add `get` command ([`f9fbf1a`](https://git.4rs.nl/awiteb/lprs/commit/f9fbf1a0b7b85638ad64287738e05ec1a1c35d25))
-  Encrypt the hole vault file ([`6f6966d`](https://git.4rs.nl/awiteb/lprs/commit/6f6966d5b25b2b5047081304f7597fe80ec95387))
    - **BC**:  The previous format is not supported after this commit, so
you must export your vaults in bit-warden format (before this commit)
and then re-invoke them (after this commit)
-  Force flag for `edit` and `add` commands ([**#42**](https://git.4rs.nl/awiteb/lprs/issues/42)) ([`add0084`](https://git.4rs.nl/awiteb/lprs/commit/add008416b37c3f8e4def891355dcbccc6786a58))
-  Lprs docs ([**#49**](https://git.4rs.nl/awiteb/lprs/issues/49)) ([`f9d36a2`](https://git.4rs.nl/awiteb/lprs/commit/f9d36a2dd781154e2dc0596874b7c9e27eff0b90))
-  Make the `name` option in `edit` & `add` as argument ([**#29**](https://git.4rs.nl/awiteb/lprs/issues/29)) ([`127f377`](https://git.4rs.nl/awiteb/lprs/commit/127f3779f8d805c7e1f5209555d8929082f85c82))
    - **BC**:  Change the `name` option to argument in `name` and `edit` commands
-  Make the username & password optional in the vault ([**#12**](https://git.4rs.nl/awiteb/lprs/issues/12)) ([`af6664d`](https://git.4rs.nl/awiteb/lprs/commit/af6664da5c08cc39cf732d64ba74de1731095723))
-  Support TOTP ([`6f83bcc`](https://git.4rs.nl/awiteb/lprs/commit/6f83bcccf94b88181d86358a922e61e3d3a2dad8))
-  Support `--verbose` flag ([**#23**](https://git.4rs.nl/awiteb/lprs/issues/23)) ([`31a68b9`](https://git.4rs.nl/awiteb/lprs/commit/31a68b927764a7eb0b38539f630b70fa258ae7aa))
-  Support `rm` and `ls` aliases ([**#22**](https://git.4rs.nl/awiteb/lprs/issues/22)) ([`791d390`](https://git.4rs.nl/awiteb/lprs/commit/791d390e636c1c29af23b343edb66279b791b121))
-  Support changing master password ([**#50**](https://git.4rs.nl/awiteb/lprs/issues/50)) ([`ced363a`](https://git.4rs.nl/awiteb/lprs/commit/ced363a37f6f64282ca1a1fb022aa3d030edff79))
-  Support completion generating ([`f022574`](https://git.4rs.nl/awiteb/lprs/commit/f022574631bfb1b6a62f95d3259617f302059781))
-  Support custom fields ([`da568ab`](https://git.4rs.nl/awiteb/lprs/commit/da568ab5e9414ef77831066eb9b09621c0fedaee))
-  Support export and import with different password ([`a6483cf`](https://git.4rs.nl/awiteb/lprs/commit/a6483cf333e6a5f3a0d48317b50c6304cfd956bb))
-  Validate args before ask for the master password ([**#17**](https://git.4rs.nl/awiteb/lprs/issues/17)) ([`b4bcaa9`](https://git.4rs.nl/awiteb/lprs/commit/b4bcaa92ca63b7c71ea5c28d5e9a6af3ecb88a91))
### Changed
-  Change 'password manager' to 'vault manager' ([`bae0cf1`](https://git.4rs.nl/awiteb/lprs/commit/bae0cf174736d9a1cd61becd20f7d87cf137249c))
-  Rename just file ([`e231352`](https://git.4rs.nl/awiteb/lprs/commit/e231352009c21886772b8f039d3e51ba0aeb7616))
-  Add a ecryption state to the vault ([`4def4aa`](https://git.4rs.nl/awiteb/lprs/commit/4def4aadb20cc367d57466dc5e88c3043e468d20))
    - **BC**:  Moving from password to vault
-  Move from GitHub to Forgejo ([`6163c3f`](https://git.4rs.nl/awiteb/lprs/commit/6163c3ff26ab81b07490a798f4047a09565ab1ac))
-  Rename `Password`s `Vault`s ([`f6aaecb`](https://git.4rs.nl/awiteb/lprs/commit/f6aaecb9cf43d7dfa3ef653ff0cd117b3197308b))
-  Use select for vaults listing ([**#19**](https://git.4rs.nl/awiteb/lprs/issues/19)) ([`83c7296`](https://git.4rs.nl/awiteb/lprs/commit/83c7296bf7bf469423f53b024cb65e608ff6c9d9))
### Fixed
-  Merge rust ci jobs into one job ([**#2**](https://git.4rs.nl/awiteb/lprs/issues/2)) ([`34eb9d1`](https://git.4rs.nl/awiteb/lprs/commit/34eb9d10f0ad514c6a7878fd8415a50f04db2be8))
-  Overflow in utils::vault_by_index_or_name function ([`40e49bf`](https://git.4rs.nl/awiteb/lprs/commit/40e49bffe4e9ecd682eb746deafd68bd088dd415))
-  Show the totp code in `get` command ([`38f6447`](https://git.4rs.nl/awiteb/lprs/commit/38f6447681d20cef313ed270cc67edc99a5ab3e2))
-  Validate all fields in `add` & `edit` ([`02bf53b`](https://git.4rs.nl/awiteb/lprs/commit/02bf53b2a1fd420bf66ac571531d060499559c29))
### Removed
-  Remove `--get` option from `list` command ([`44b5b3e`](https://git.4rs.nl/awiteb/lprs/commit/44b5b3e09b6c653b0d201e268878718cfa507209))
    - **BC**:  The deletion was in favor `get` command, which is better

## [1.2.1](https://git.4rs.nl/awiteb/lprs/compare/v1.2.0..v1.2.1) - 2024-01-07

## [1.2.0](https://git.4rs.nl/awiteb/lprs/compare/v1.1.0..v1.2.0) - 2024-01-07

## [1.1.0](https://git.4rs.nl/awiteb/lprs/compare/v1.0.0..v1.1.0) - 2024-01-03

## [1.0.0](https://git.4rs.nl/awiteb/lprs/compare/v0.1.0..v1.0.0) - 2023-12-31

## 0.1.0 - 2023-12-23

This changelog was generated by [git-cliff](https://github.com/orhun/git-cliff)
