help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

dev: ## Develop the app.
	pnpm run dev

fmt:  ## Format the codebase.
	pnpx prettier --config configs/prettier.config.js --ignore-path configs/prettierignore --write .

fmt_check: ## Check is the codebase properly formatted.
	pnpx prettier --config configs/prettier.config.js --ignore-path configs/prettierignore --check .

build: ## Build the app.
	pnpm run build

comply: fmt  ## Tasks to make the code-base comply with the rules. Mostly used locally or in git hooks.
check:  fmt_check ## Check if the repository comply with the rules. Mostly used in CI.
