#!/usr/bin/env bash
set -Eeuo pipefail
[[ "${DEBUG:-0}" == "1" ]] && set -x

is_git_repo() { git rev-parse --is-inside-work-tree >/dev/null 2>&1; }

classify_prefix() {
  local status="$1" path="$2"
  case "$path" in
    README*|*.md|*.rst|docs/*) echo "docs"; return;;
    lessons/*)                 echo "content"; return;;
    *tests.rs|tests/*|*/test/*|*_test.rs) echo "test"; return;;
    Cargo.toml|Cargo.lock|.gitignore|.editorconfig|.gitattributes|.github/*|.vscode/*|.idea/*)
                               echo "chore"; return;;
    src/*|*.rs)
      [[ "$status" =~ ^A ]] && echo "feat" || echo "chore"; return;;
  esac
  echo "chore"
}

build_message() {
  local status="$1" from="$2" to="${3:-$2}"
  [[ "$to" == "README.md" ]] && { echo "docs: update README (GUI-only; beta/WIP)"; return; }
  local prefix; prefix="$(classify_prefix "$status" "$to")"
  case "$status" in
    D*) echo "$prefix: remove $to";;
    R*|C*) echo "$prefix: rename $from -> $to";;
    A*) echo "$prefix: add $to";;
    M*|T*) echo "$prefix: update $to";;
    *) echo "$prefix: update $to";;
  esac
}

main() {
  if ! is_git_repo; then echo "Not a git repository." >&2; exit 1; fi

  git add -A
  if git diff --cached --quiet; then echo "No staged changes. Nothing to commit."; exit 0; fi

  current_branch="$(git rev-parse --abbrev-ref HEAD)"

  # Parse newline-delimited status; fields are TAB-separated
  git diff --cached --name-status | while IFS=$'\t' read -r status path1 path2 || [[ -n "${status:-}" ]]; do
    # Skip empty lines
    [[ -z "${status:-}" ]] && continue

    if [[ "$status" =~ ^[RC] ]]; then
      from="$path1"; to="$path2"
      msg="$(build_message "$status" "$from" "$to")"
      echo "Committing (rename/copy): $msg"
      if [[ "${DRY_RUN:-0}" != "1" ]]; then
        if git diff --cached --quiet -- "$from" "$to"; then
          echo "  Skip (no staged diff for rename): $from -> $to"
        else
          git commit -m "$msg" -- "$from" "$to"
        fi
      fi
    else
      path="$path1"
      msg="$(build_message "$status" "$path")"
      echo "Committing: $msg"
      if [[ "${DRY_RUN:-0}" != "1" ]]; then
        if git diff --cached --quiet -- "$path"; then
          echo "  Skip (no staged diff): $path"
        else
          git commit -m "$msg" -- "$path"
        fi
      fi
    fi
  done

  echo
  echo "All commits created on branch: $current_branch"
  echo "Push with:"
  echo "  git push -u origin $current_branch"
}

trap 'echo "Error on line $LINENO. Re-run with DEBUG=1 for more detail." >&2' ERR
main "$@"