- [Commit specification](#commit-specification)
  - [Process finalizing commit](#process-finalizing-commit)

# Commit specification
## Process finalizing commit
This process is seperated from the consensus engine. It creates commit and apply block.
Applying block:
- block operation, save block with commit
- block executor, apply block
