#!/bin/bash
git add -A
git add --force yewapp/rel
git commit -m $(date +%s)
GIT_SSH_COMMAND="ssh -i ~/.ssh/id_gitpro" git push origin main

